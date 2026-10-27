use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use realfft::RealFftPlanner;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

/// Shared state for the latest detected frequency
type FreqState = Arc<Mutex<Option<f32>>>;

pub struct AudioEngine {
    latest_freq: FreqState,
    running: Arc<AtomicBool>,
    _stream: Option<cpal::Stream>,
}

impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No default input device found"))?;

        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let latest_freq: FreqState = Arc::new(Mutex::new(None));
        let running = Arc::new(AtomicBool::new(false));

        let freq_clone = Arc::clone(&latest_freq);
        let running_clone = Arc::clone(&running);

        // Analysis parameters
        let buffer_size = 4096usize;
        let mut sample_buffer: Vec<f32> = Vec::with_capacity(buffer_size * 2);

        // FFT planner lives inside the callback via move (recreated is fine for this use)
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if !running_clone.load(Ordering::Relaxed) {
                            return;
                        }

                        // Mono mix / first channel
                        for frame in data.chunks(channels) {
                            sample_buffer.push(frame[0]);
                        }

                        while sample_buffer.len() >= buffer_size {
                            let chunk: Vec<f32> = sample_buffer.drain(..buffer_size).collect();

                            // Hann window
                            let mut windowed = chunk;
                            for (i, s) in windowed.iter_mut().enumerate() {
                                let phase = 2.0 * std::f32::consts::PI * i as f32
                                    / (buffer_size as f32 - 1.0);
                                let w = 0.5 * (1.0 - phase.cos());
                                *s *= w;
                            }

                            // FFT
                            let mut planner = RealFftPlanner::<f32>::new();
                            let r2c = planner.plan_fft_forward(buffer_size);
                            let mut spectrum = r2c.make_output_vec();
                            let mut scratch = r2c.make_scratch_vec();

                            if r2c
                                .process_with_scratch(&mut windowed, &mut spectrum, &mut scratch)
                                .is_err()
                            {
                                continue;
                            }

                            // Peak in guitar-ish range
                            let bin_hz = sample_rate / buffer_size as f32;
                            let min_bin = (70.0 / bin_hz) as usize;
                            let max_bin =
                                ((1200.0 / bin_hz) as usize).min(buffer_size / 2 - 1);

                            let mut max_mag = 0.0f32;
                            let mut max_bin_idx = 0usize;

                            for i in min_bin..=max_bin {
                                let re = spectrum[i].re;
                                let im = spectrum[i].im;
                                let mag = (re * re + im * im).sqrt();
                                if mag > max_mag {
                                    max_mag = mag;
                                    max_bin_idx = i;
                                }
                            }

                            // Simple magnitude gate (adjust if too sensitive / insensitive)
                            if max_mag > 8.0 {
                                let freq = max_bin_idx as f32 * bin_hz;
                                if let Ok(mut guard) = freq_clone.lock() {
                                    *guard = Some(freq);
                                }
                            } else if let Ok(mut guard) = freq_clone.lock() {
                                *guard = None;
                            }
                        }
                    },
                    |err| eprintln!("[audio] stream error: {err}"),
                    None,
                )?
            }
            other => {
                return Err(anyhow::anyhow!(
                    "Unsupported sample format: {:?}. Need F32.",
                    other
                ));
            }
        };

        Ok(Self {
            latest_freq,
            running,
            _stream: Some(stream),
        })
    }

    pub fn dummy() -> Self {
        Self {
            latest_freq: Arc::new(Mutex::new(None)),
            running: Arc::new(AtomicBool::new(false)),
            _stream: None,
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.running.store(true, Ordering::SeqCst);
        if let Some(ref stream) = self._stream {
            stream.play()?;
        }
        Ok(())
    }

    pub fn latest_frequency(&self) -> Option<f32> {
        self.latest_freq
            .lock()
            .ok()
            .and_then(|guard| *guard)
    }
}

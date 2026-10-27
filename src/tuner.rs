/// Standard guitar string frequencies (A4 = 440 Hz)
pub const STANDARD_TUNING: [(&str, f32); 6] = [
    ("E2 (6th)", 82.41),
    ("A2 (5th)", 110.00),
    ("D3 (4th)", 146.83),
    ("G3 (3rd)", 196.00),
    ("B3 (2nd)", 246.94),
    ("E4 (1st)", 329.63),
];

pub struct TunerState {
    pub frequency: Option<f32>,
    pub note_name: String,
    pub cents: f32,
    pub closest_string: usize,
    pub is_in_tune: bool,
}

impl TunerState {
    pub fn new() -> Self {
        Self {
            frequency: None,
            note_name: "—".into(),
            cents: 0.0,
            closest_string: 0,
            is_in_tune: false,
        }
    }

    pub fn update(&mut self, freq: f32) {
        self.frequency = Some(freq);

        // Find closest standard string
        let mut best_diff = f32::MAX;
        let mut best_idx = 0;
        for (i, (_, target)) in STANDARD_TUNING.iter().enumerate() {
            let cents = 1200.0 * (freq / target).log2();
            let abs = cents.abs();
            if abs < best_diff {
                best_diff = abs;
                best_idx = i;
            }
        }

        self.closest_string = best_idx;
        let target = STANDARD_TUNING[best_idx].1;
        self.cents = 1200.0 * (freq / target).log2();
        self.is_in_tune = self.cents.abs() < 5.0; // ±5 cents is "in tune"

        // Note name from frequency (simplified chromatic)
        self.note_name = freq_to_note(freq);
    }
}

fn freq_to_note(freq: f32) -> String {
    // A4 = 440
    let a4 = 440.0;
    let n = (12.0 * (freq / a4).log2()).round() as i32;
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    // MIDI note of A4 is 69
    let midi = 69 + n;
    let name = note_names[((midi % 12) + 12) as usize % 12];
    let octave = (midi / 12) - 1;
    format!("{name}{octave}")
}

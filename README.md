# Guitar Mentor 🎸

**Offline guitar tuner + chord teacher + rock song library**  
Built in pure Rust for macOS (optimized for Apple Silicon M1/M2/M3).

## Features

- **Real-time Tuner** — uses your microphone, shows note name, frequency and cents deviation with a visual needle.
- **Chord Library** — open chords, power chords (great for electric/rock), barre chords with clear fingering diagrams.
- **Rock Songs** — 20 classic rock songs with chords used, progressions, section breakdowns and playing tips.
- **100% Offline** — no internet, no accounts, no tracking.
- **Beautiful dark UI** — modern, clean and focused on the instrument.

## Requirements

- macOS (Apple Silicon recommended)
- Rust toolchain (install from https://rustup.rs)
- Microphone permission (macOS will ask on first run)

## Build & Run

```bash
# Clone or copy this folder, then:
cd guitar_mentor

# Build in release mode (recommended for performance)
cargo build --release

# Run
./target/release/guitar_mentor
```

Or simply:

```bash
cargo run --release
```

## First launch notes

1. macOS will ask for **Microphone** permission — please allow it so the tuner can hear your guitar.
2. For best tuner results: play in a quiet room, pluck cleanly and let the string ring.
3. The FFT-based detector works well for single notes. It is monophonic (one note at a time).

## Project structure

```
src/
  main.rs      → App entry + dark theme
  audio.rs     → cpal microphone capture + FFT pitch detection
  tuner.rs     → Note/cents calculation + standard tuning targets
  chords.rs    → Chord shapes & library
  songs.rs     → Offline rock song database
  ui.rs        → All panels (Tuner / Chords / Songs / About)
```

## Extending

- Add more chords in `chords.rs`
- Add more songs in `songs.rs` (just follow the existing `Song` struct)
- The UI is pure `egui` — easy to restyle or add new tabs

## Technical notes

- Audio: `cpal` (CoreAudio on macOS)
- Pitch detection: windowed FFT + peak picking in the guitar range
- GUI: `eframe` / `egui` (immediate mode, GPU accelerated)
- Fully native — no Electron, no webview

Enjoy practicing!

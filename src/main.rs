#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod chords;
mod songs;
mod tuner;
mod ui;

use eframe::egui;
use std::sync::{Arc, Mutex};

use audio::AudioEngine;
use chords::ChordLibrary;
use songs::SongLibrary;
use tuner::TunerState;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("Guitar Mentor — Offline Tuner & Rock Chord Teacher"),
        ..Default::default()
    };

    eframe::run_native(
        "Guitar Mentor",
        options,
        Box::new(|cc| {
            // Beautiful dark theme
            let mut visuals = egui::Visuals::dark();
            visuals.window_fill = egui::Color32::from_rgb(18, 18, 24);
            visuals.panel_fill = egui::Color32::from_rgb(22, 22, 30);
            visuals.override_text_color = Some(egui::Color32::from_rgb(230, 230, 240));
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 40);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 40, 55);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(55, 55, 75);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(70, 90, 140);
            visuals.selection.bg_fill = egui::Color32::from_rgb(80, 120, 200);
            cc.egui_ctx.set_visuals(visuals);

            // Slightly larger base font
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::new(26.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            );
            cc.egui_ctx.set_style(style);

            Ok(Box::new(GuitarMentorApp::new(cc)))
        }),
    )
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Tuner,
    Chords,
    Songs,
    About,
}

struct GuitarMentorApp {
    tab: Tab,
    audio: Arc<Mutex<AudioEngine>>,
    tuner: TunerState,
    chord_lib: ChordLibrary,
    song_lib: SongLibrary,
    selected_chord: String,
    selected_song_idx: Option<usize>,
    show_chord_diagram: bool,
}

impl GuitarMentorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let audio = Arc::new(Mutex::new(AudioEngine::new().unwrap_or_else(|e| {
            eprintln!("Audio init warning: {e}. Tuner will be limited.");
            AudioEngine::dummy()
        })));

        // Start listening
        if let Ok(mut eng) = audio.lock() {
            let _ = eng.start();
        }

        Self {
            tab: Tab::Tuner,
            audio,
            tuner: TunerState::new(),
            chord_lib: ChordLibrary::new(),
            song_lib: SongLibrary::new(),
            selected_chord: "E".to_string(),
            selected_song_idx: Some(0),
            show_chord_diagram: true,
        }
    }
}

impl eframe::App for GuitarMentorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Continuous repaint for live tuner
        ctx.request_repaint();

        // Pull latest pitch from audio engine
        if let Ok(eng) = self.audio.lock() {
            if let Some(freq) = eng.latest_frequency() {
                self.tuner.update(freq);
            }
        }

        // Top bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.heading("🎸 Guitar Mentor");
                ui.add_space(20.0);

                ui.selectable_value(&mut self.tab, Tab::Tuner, "  Tuner  ");
                ui.selectable_value(&mut self.tab, Tab::Chords, "  Chords  ");
                ui.selectable_value(&mut self.tab, Tab::Songs, "  Rock Songs  ");
                ui.selectable_value(&mut self.tab, Tab::About, "  About  ");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("100% Offline • Apple Silicon Ready")
                            .color(egui::Color32::from_rgb(120, 180, 255))
                            .size(13.0),
                    );
                });
            });
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Tuner => ui::tuner_panel(ui, &mut self.tuner),
                Tab::Chords => ui::chords_panel(
                    ui,
                    &self.chord_lib,
                    &mut self.selected_chord,
                    &mut self.show_chord_diagram,
                ),
                Tab::Songs => ui::songs_panel(
                    ui,
                    &self.song_lib,
                    &mut self.selected_song_idx,
                    &self.chord_lib,
                ),
                Tab::About => ui::about_panel(ui),
            }
        });
    }
}

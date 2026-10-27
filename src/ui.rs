use crate::chords::{ChordLibrary, ChordShape};
use crate::songs::SongLibrary;
use crate::tuner::{TunerState, STANDARD_TUNING};
use egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};

pub fn tuner_panel(ui: &mut egui::Ui, tuner: &mut TunerState) {
    ui.add_space(10.0);

    ui.vertical_centered(|ui| {
        ui.heading("Guitar Tuner");
        ui.label(
            egui::RichText::new("Play a string — the app listens through your microphone")
                .color(Color32::from_rgb(160, 160, 180))
                .size(14.0),
        );
    });

    ui.add_space(20.0);

    // Big frequency + note display
    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() * 0.15);

        ui.vertical(|ui| {
            let note = &tuner.note_name;
            let color = if tuner.is_in_tune {
                Color32::from_rgb(80, 220, 120)
            } else if tuner.frequency.is_some() {
                Color32::from_rgb(255, 180, 60)
            } else {
                Color32::from_rgb(140, 140, 160)
            };

            ui.label(
                egui::RichText::new(note)
                    .size(72.0)
                    .color(color)
                    .strong(),
            );

            if let Some(freq) = tuner.frequency {
                ui.label(
                    egui::RichText::new(format!("{:.1} Hz", freq))
                        .size(22.0)
                        .color(Color32::from_rgb(180, 180, 200)),
                );
            } else {
                ui.label(
                    egui::RichText::new("Listening…")
                        .size(22.0)
                        .color(Color32::from_rgb(120, 120, 140)),
                );
            }
        });

        ui.add_space(40.0);

        // Cents needle visual
        let needle_height = 180.0;
        let needle_width = 280.0;
        let (rect, _response) = ui.allocate_exact_size(
            Vec2::new(needle_width, needle_height),
            Sense::hover(),
        );

        let painter = ui.painter();
        let center = rect.center();

        // Background arc area
        painter.rect_filled(
            rect.shrink(4.0),
            12.0,
            Color32::from_rgb(28, 28, 38),
        );

        // Scale marks
        for i in -5..=5 {
            let t = i as f32 / 5.0;
            let x = center.x + t * (needle_width * 0.4);
            let h = if i == 0 { 28.0 } else { 16.0 };
            painter.line_segment(
                [
                    Pos2::new(x, center.y + 30.0),
                    Pos2::new(x, center.y + 30.0 - h),
                ],
                Stroke::new(2.0, Color32::from_rgb(90, 90, 110)),
            );
        }

        // Center zero line
        painter.line_segment(
            [
                Pos2::new(center.x, center.y - 40.0),
                Pos2::new(center.x, center.y + 40.0),
            ],
            Stroke::new(2.5, Color32::from_rgb(100, 160, 255)),
        );

        // Needle
        let cents_clamped = tuner.cents.clamp(-50.0, 50.0);
        let needle_x = center.x + (cents_clamped / 50.0) * (needle_width * 0.4);
        let needle_color = if tuner.is_in_tune {
            Color32::from_rgb(80, 230, 130)
        } else {
            Color32::from_rgb(255, 140, 50)
        };

        painter.line_segment(
            [
                Pos2::new(needle_x, center.y - 55.0),
                Pos2::new(needle_x, center.y + 45.0),
            ],
            Stroke::new(4.0, needle_color),
        );

        // Cents text
        let cents_text = if tuner.frequency.is_some() {
            format!("{:+.0} cents", tuner.cents)
        } else {
            "—".into()
        };
        painter.text(
            Pos2::new(center.x, rect.bottom() - 18.0),
            egui::Align2::CENTER_CENTER,
            cents_text,
            egui::FontId::proportional(18.0),
            Color32::from_rgb(200, 200, 220),
        );
    });

    ui.add_space(24.0);

    // String targets
    ui.separator();
    ui.add_space(8.0);
    ui.label(egui::RichText::new("Standard Tuning Targets").strong().size(16.0));
    ui.add_space(8.0);

    ui.horizontal_wrapped(|ui| {
        for (i, (name, freq)) in STANDARD_TUNING.iter().enumerate() {
            let is_closest = i == tuner.closest_string && tuner.frequency.is_some();
            let bg = if is_closest {
                if tuner.is_in_tune {
                    Color32::from_rgb(40, 90, 60)
                } else {
                    Color32::from_rgb(90, 60, 30)
                }
            } else {
                Color32::from_rgb(35, 35, 48)
            };

            egui::Frame::none()
                .fill(bg)
                .rounding(8.0)
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(*name).size(14.0).strong());
                        ui.label(
                            egui::RichText::new(format!("{:.1} Hz", freq))
                                .size(12.0)
                                .color(Color32::from_rgb(160, 160, 180)),
                        );
                    });
                });
        }
    });

    ui.add_space(16.0);
    ui.label(
        egui::RichText::new("Tip: Pluck the string cleanly and let it ring. Avoid background noise for best results.")
            .italics()
            .color(Color32::from_rgb(130, 130, 150))
            .size(13.0),
    );
}

pub fn chords_panel(
    ui: &mut egui::Ui,
    lib: &ChordLibrary,
    selected: &mut String,
    show_diagram: &mut bool,
) {
    ui.add_space(8.0);
    ui.heading("Chord Library");
    ui.label("Click any chord to see the fingering diagram. All offline.");

    ui.add_space(12.0);

    // Chord selector grid
    ui.horizontal_wrapped(|ui| {
        for name in lib.all_names() {
            let is_sel = name == selected;
            let btn = egui::Button::new(
                egui::RichText::new(name)
                    .size(15.0)
                    .color(if is_sel {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(210, 210, 230)
                    }),
            )
            .fill(if is_sel {
                Color32::from_rgb(70, 110, 200)
            } else {
                Color32::from_rgb(40, 40, 55)
            })
            .rounding(6.0);

            if ui.add(btn).clicked() {
                *selected = name.clone();
                *show_diagram = true;
            }
        }
    });

    ui.add_space(16.0);
    ui.separator();

    if let Some(chord) = lib.get(selected) {
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.heading(&chord.name);
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(format!("{} • Difficulty {}", chord.category, "★".repeat(chord.difficulty as usize)))
                    .color(Color32::from_rgb(180, 160, 100)),
            );
        });

        ui.add_space(12.0);

        // Draw the chord diagram
        draw_chord_diagram(ui, chord);

        ui.add_space(16.0);
        ui.label(
            egui::RichText::new("Fingering tip: numbers show which finger to use (1=index, 2=middle, 3=ring, 4=pinky). X = mute string, 0 = open string.")
                .color(Color32::from_rgb(140, 140, 160))
                .size(13.0),
        );
    }
}

fn draw_chord_diagram(ui: &mut egui::Ui, chord: &ChordShape) {
    let width = 260.0;
    let height = 320.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let painter = ui.painter();

    // Background
    painter.rect_filled(rect, 12.0, Color32::from_rgb(26, 26, 36));

    let left = rect.left() + 40.0;
    let top = rect.top() + 50.0;
    let string_spacing = 36.0;
    let fret_spacing = 42.0;
    let num_frets = 5;

    // Nut
    painter.line_segment(
        [
            Pos2::new(left, top),
            Pos2::new(left + 5.0 * string_spacing, top),
        ],
        Stroke::new(6.0, Color32::from_rgb(220, 220, 230)),
    );

    // Strings (vertical)
    for s in 0..6 {
        let x = left + s as f32 * string_spacing;
        painter.line_segment(
            [
                Pos2::new(x, top),
                Pos2::new(x, top + num_frets as f32 * fret_spacing),
            ],
            Stroke::new(1.8, Color32::from_rgb(160, 160, 180)),
        );
    }

    // Frets (horizontal)
    for f in 1..=num_frets {
        let y = top + f as f32 * fret_spacing;
        painter.line_segment(
            [
                Pos2::new(left, y),
                Pos2::new(left + 5.0 * string_spacing, y),
            ],
            Stroke::new(1.5, Color32::from_rgb(90, 90, 110)),
        );
    }

    // Fret numbers on the side
    for f in 1..=num_frets {
        let y = top + (f as f32 - 0.5) * fret_spacing;
        painter.text(
            Pos2::new(left - 18.0, y),
            egui::Align2::CENTER_CENTER,
            f.to_string(),
            egui::FontId::proportional(13.0),
            Color32::from_rgb(130, 130, 150),
        );
    }

    // Open / mute indicators above nut
    for (s, &fret) in chord.frets.iter().enumerate() {
        let x = left + s as f32 * string_spacing;
        if fret == 0 {
            painter.circle_stroke(
                Pos2::new(x, top - 18.0),
                8.0,
                Stroke::new(2.0, Color32::from_rgb(100, 200, 120)),
            );
        } else if fret < 0 {
            // X for muted
            let r = 7.0;
            painter.line_segment(
                [Pos2::new(x - r, top - 18.0 - r), Pos2::new(x + r, top - 18.0 + r)],
                Stroke::new(2.2, Color32::from_rgb(220, 100, 100)),
            );
            painter.line_segment(
                [Pos2::new(x + r, top - 18.0 - r), Pos2::new(x - r, top - 18.0 + r)],
                Stroke::new(2.2, Color32::from_rgb(220, 100, 100)),
            );
        }
    }

    // Finger dots
    for (s, &fret) in chord.frets.iter().enumerate() {
        if fret > 0 {
            let x = left + s as f32 * string_spacing;
            let y = top + (fret as f32 - 0.5) * fret_spacing;
            painter.circle_filled(Pos2::new(x, y), 14.0, Color32::from_rgb(70, 130, 230));

            // Finger number
            let finger = chord.fingers[s];
            if finger > 0 {
                painter.text(
                    Pos2::new(x, y),
                    egui::Align2::CENTER_CENTER,
                    finger.to_string(),
                    egui::FontId::proportional(14.0),
                    Color32::WHITE,
                );
            }
        }
    }

    // String labels at bottom
    let labels = ["E", "A", "D", "G", "B", "e"];
    for (s, label) in labels.iter().enumerate() {
        let x = left + s as f32 * string_spacing;
        painter.text(
            Pos2::new(x, top + num_frets as f32 * fret_spacing + 18.0),
            egui::Align2::CENTER_CENTER,
            *label,
            egui::FontId::proportional(13.0),
            Color32::from_rgb(160, 160, 180),
        );
    }
}

pub fn songs_panel(
    ui: &mut egui::Ui,
    lib: &SongLibrary,
    selected_idx: &mut Option<usize>,
    chord_lib: &ChordLibrary,
) {
    ui.add_space(6.0);
    ui.heading("Rock Songs — Chord Charts");
    ui.label("Fully offline. Select a song to see chords, progression and tips.");

    ui.add_space(10.0);

    // Left list + right detail
    ui.columns(2, |cols| {
        // Song list
        egui::ScrollArea::vertical()
            .id_salt("song_list")
            .show(&mut cols[0], |ui| {
                for (i, song) in lib.songs.iter().enumerate() {
                    let is_sel = *selected_idx == Some(i);
                    let bg = if is_sel {
                        Color32::from_rgb(50, 70, 120)
                    } else {
                        Color32::from_rgb(30, 30, 42)
                    };

                    let response = egui::Frame::none()
                        .fill(bg)
                        .rounding(8.0)
                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("{}", song.title))
                                        .strong()
                                        .size(14.5),
                                );
                            });
                            ui.label(
                                egui::RichText::new(format!("{} • {}", song.artist, song.year))
                                    .size(12.0)
                                    .color(Color32::from_rgb(150, 150, 170)),
                            );
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("Key: {}", song.key))
                                        .size(11.5)
                                        .color(Color32::from_rgb(120, 160, 220)),
                                );
                                ui.label(
                                    egui::RichText::new("★".repeat(song.difficulty as usize))
                                        .size(11.5)
                                        .color(Color32::from_rgb(220, 180, 80)),
                                );
                            });
                        })
                        .response
                        .interact(Sense::click());

                    if response.clicked() {
                        *selected_idx = Some(i);
                    }
                    ui.add_space(4.0);
                }
            });

        // Detail
        cols[1].add_space(4.0);
        if let Some(idx) = *selected_idx {
            if let Some(song) = lib.songs.get(idx) {
                egui::ScrollArea::vertical()
                    .id_salt("song_detail")
                    .show(&mut cols[1], |ui| {
                        ui.heading(song.title);
                        ui.label(
                            egui::RichText::new(format!("{}  ({})", song.artist, song.year))
                                .size(15.0)
                                .color(Color32::from_rgb(160, 160, 190)),
                        );

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Key:").strong());
                            ui.label(song.key);
                            ui.add_space(12.0);
                            ui.label(egui::RichText::new("Difficulty:").strong());
                            ui.label("★".repeat(song.difficulty as usize));
                        });

                        ui.add_space(12.0);
                        ui.label(egui::RichText::new("Chords used").strong().size(15.0));
                        ui.horizontal_wrapped(|ui| {
                            for c in song.chords_used {
                                let exists = chord_lib.get(c).is_some();
                                let color = if exists {
                                    Color32::from_rgb(100, 180, 255)
                                } else {
                                    Color32::from_rgb(180, 140, 100)
                                };
                                ui.label(
                                    egui::RichText::new(format!("  {}  ", c))
                                        .size(15.0)
                                        .color(color)
                                        .strong(),
                                );
                            }
                        });

                        ui.add_space(12.0);
                        ui.label(egui::RichText::new("Progression").strong().size(15.0));
                        ui.label(
                            egui::RichText::new(song.progression)
                                .size(16.0)
                                .color(Color32::from_rgb(200, 220, 160)),
                        );

                        ui.add_space(14.0);
                        ui.label(egui::RichText::new("Sections").strong().size(15.0));
                        for sec in song.sections {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(sec.name)
                                    .strong()
                                    .color(Color32::from_rgb(140, 170, 255)),
                            );
                            ui.label(
                                egui::RichText::new(sec.chords)
                                    .size(15.0)
                                    .monospace()
                                    .color(Color32::from_rgb(220, 220, 230)),
                            );
                        }

                        ui.add_space(16.0);
                        egui::Frame::none()
                            .fill(Color32::from_rgb(35, 40, 55))
                            .rounding(8.0)
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("💡 Tip").strong());
                                ui.label(song.tips);
                            });
                    });
            }
        } else {
            cols[1].label("Select a song from the list →");
        }
    });
}

pub fn about_panel(ui: &mut egui::Ui) {
    ui.add_space(20.0);
    ui.vertical_centered(|ui| {
        ui.heading("Guitar Mentor");
        ui.add_space(8.0);
        ui.label("Offline guitar companion built with Rust");
        ui.add_space(16.0);

        ui.label("Features:");
        ui.label("• Real-time chromatic / guitar tuner (microphone)");
        ui.label("• Open, power & barre chord diagrams");
        ui.label("• 20 classic rock songs with chord charts");
        ui.label("• 100% offline — no internet required");
        ui.label("• Native performance on Apple Silicon (M1/M2/M3)");
        ui.add_space(16.0);

        ui.label(
            egui::RichText::new("Built for learning and playing — acoustic & electric")
                .italics()
                .color(Color32::from_rgb(160, 160, 180)),
        );
        ui.add_space(12.0);
        ui.label("Tip: Grant microphone permission when macOS asks.");
    });
}

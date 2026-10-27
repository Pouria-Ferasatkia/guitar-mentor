use std::collections::HashMap;

/// Fingering: for each string (0=low E to 5=high E), fret number.
/// -1 = muted, 0 = open.
#[derive(Clone, Debug)]
pub struct ChordShape {
    pub name: String,
    pub frets: [i8; 6], // low E → high E
    pub fingers: [u8; 6], // suggested finger numbers (0 = open/mute)
    pub difficulty: u8, // 1-5
    pub category: &'static str,
}

pub struct ChordLibrary {
    chords: HashMap<String, ChordShape>,
    pub order: Vec<String>,
}

impl ChordLibrary {
    pub fn new() -> Self {
        let mut chords = HashMap::new();
        let mut order = Vec::new();

        // Open chords – most important for beginners & rock
        add(&mut chords, &mut order, "E", [0, 2, 2, 1, 0, 0], [0, 2, 3, 1, 0, 0], 1, "Open");
        add(&mut chords, &mut order, "A", [0, 0, 2, 2, 2, 0], [0, 0, 1, 2, 3, 0], 1, "Open");
        add(&mut chords, &mut order, "D", [-1, 0, 0, 2, 3, 2], [0, 0, 0, 1, 3, 2], 1, "Open");
        add(&mut chords, &mut order, "G", [3, 2, 0, 0, 0, 3], [2, 1, 0, 0, 0, 3], 1, "Open");
        add(&mut chords, &mut order, "C", [-1, 3, 2, 0, 1, 0], [0, 3, 2, 0, 1, 0], 1, "Open");
        add(&mut chords, &mut order, "Em", [0, 2, 2, 0, 0, 0], [0, 2, 3, 0, 0, 0], 1, "Open");
        add(&mut chords, &mut order, "Am", [0, 0, 2, 2, 1, 0], [0, 0, 2, 3, 1, 0], 1, "Open");
        add(&mut chords, &mut order, "Dm", [-1, 0, 0, 2, 3, 1], [0, 0, 0, 2, 3, 1], 2, "Open");

        // Power chords (essential for rock/electric)
        add(&mut chords, &mut order, "E5", [0, 2, 2, -1, -1, -1], [0, 1, 2, 0, 0, 0], 1, "Power");
        add(&mut chords, &mut order, "A5", [-1, 0, 2, 2, -1, -1], [0, 0, 1, 2, 0, 0], 1, "Power");
        add(&mut chords, &mut order, "D5", [-1, -1, 0, 2, 3, -1], [0, 0, 0, 1, 2, 0], 1, "Power");
        add(&mut chords, &mut order, "G5", [3, 5, 5, -1, -1, -1], [1, 3, 4, 0, 0, 0], 2, "Power");
        add(&mut chords, &mut order, "C5", [-1, 3, 5, 5, -1, -1], [0, 1, 3, 4, 0, 0], 2, "Power");
        add(&mut chords, &mut order, "B5", [-1, 2, 4, 4, -1, -1], [0, 1, 3, 4, 0, 0], 2, "Power");

        // Barre / movable
        add(&mut chords, &mut order, "F", [1, 3, 3, 2, 1, 1], [1, 3, 4, 2, 1, 1], 3, "Barre");
        add(&mut chords, &mut order, "Bm", [-1, 2, 4, 4, 3, 2], [0, 1, 3, 4, 2, 1], 3, "Barre");
        add(&mut chords, &mut order, "F#m", [2, 4, 4, 2, 2, 2], [1, 3, 4, 1, 1, 1], 3, "Barre");

        // Common rock extras
        add(&mut chords, &mut order, "A7", [0, 0, 2, 0, 2, 0], [0, 0, 1, 0, 2, 0], 2, "Open");
        add(&mut chords, &mut order, "E7", [0, 2, 0, 1, 0, 0], [0, 2, 0, 1, 0, 0], 1, "Open");
        add(&mut chords, &mut order, "D7", [-1, 0, 0, 2, 1, 2], [0, 0, 0, 2, 1, 3], 2, "Open");
        add(&mut chords, &mut order, "G7", [3, 2, 0, 0, 0, 1], [3, 2, 0, 0, 0, 1], 2, "Open");
        add(&mut chords, &mut order, "Cadd9", [-1, 3, 2, 0, 3, 0], [0, 2, 1, 0, 3, 0], 2, "Open");
        add(&mut chords, &mut order, "Asus2", [0, 0, 2, 2, 0, 0], [0, 0, 1, 2, 0, 0], 1, "Open");
        add(&mut chords, &mut order, "Dsus4", [-1, 0, 0, 2, 3, 3], [0, 0, 0, 1, 2, 3], 2, "Open");
        add(&mut chords, &mut order, "Em7", [0, 2, 0, 0, 0, 0], [0, 2, 0, 0, 0, 0], 1, "Open");

        Self { chords, order }
    }

    pub fn get(&self, name: &str) -> Option<&ChordShape> {
        self.chords.get(name)
    }

    pub fn all_names(&self) -> &[String] {
        &self.order
    }
}

fn add(
    map: &mut HashMap<String, ChordShape>,
    order: &mut Vec<String>,
    name: &str,
    frets: [i8; 6],
    fingers: [u8; 6],
    difficulty: u8,
    category: &'static str,
) {
    map.insert(
        name.to_string(),
        ChordShape {
            name: name.to_string(),
            frets,
            fingers,
            difficulty,
            category,
        },
    );
    order.push(name.to_string());
}

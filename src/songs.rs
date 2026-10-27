#[derive(Clone)]
pub struct Song {
    pub title: &'static str,
    pub artist: &'static str,
    pub year: u16,
    pub key: &'static str,
    pub difficulty: u8, // 1-5
    pub chords_used: &'static [&'static str],
    pub progression: &'static str, // human readable
    pub sections: &'static [SongSection],
    pub tips: &'static str,
}

#[derive(Clone)]
pub struct SongSection {
    pub name: &'static str,
    pub chords: &'static str,
}

pub struct SongLibrary {
    pub songs: Vec<Song>,
}

impl SongLibrary {
    pub fn new() -> Self {
        let songs = vec![
            Song {
                title: "Knockin' on Heaven's Door",
                artist: "Bob Dylan / Guns N' Roses",
                year: 1973,
                key: "G",
                difficulty: 1,
                chords_used: &["G", "D", "Am", "C"],
                progression: "G – D – Am | G – D – C",
                sections: &[
                    SongSection { name: "Verse", chords: "G   D   Am          G   D   C" },
                    SongSection { name: "Chorus", chords: "G   D   C          G   D   C" },
                ],
                tips: "Slow strumming. Perfect first song. Practice clean G→D changes.",
            },
            Song {
                title: "Sweet Home Alabama",
                artist: "Lynyrd Skynyrd",
                year: 1974,
                key: "D",
                difficulty: 2,
                chords_used: &["D", "C", "G"],
                progression: "D – C – G (repeat)",
                sections: &[
                    SongSection { name: "Main Riff / Verse", chords: "D   C   G          D   C   G" },
                    SongSection { name: "Chorus", chords: "D   C   G          D   C   G" },
                ],
                tips: "Classic rock groove. Accent the D chord. Great for rhythm practice.",
            },
            Song {
                title: "Wonderwall",
                artist: "Oasis",
                year: 1995,
                key: "Em (capo 2)",
                difficulty: 2,
                chords_used: &["Em", "G", "D", "A7", "C"],
                progression: "Em – G – D – A7sus4  /  C – D – Em",
                sections: &[
                    SongSection { name: "Intro / Verse", chords: "Em  G  D  A7sus4" },
                    SongSection { name: "Chorus", chords: "C   D   Em          C   D   G" },
                ],
                tips: "Use a capo on 2nd fret for original key. Strum pattern is key.",
            },
            Song {
                title: "Smoke on the Water",
                artist: "Deep Purple",
                year: 1972,
                key: "G (power chords)",
                difficulty: 1,
                chords_used: &["G5", "Bb5", "C5"],
                progression: "Power chord riff in G",
                sections: &[
                    SongSection { name: "Main Riff", chords: "G5  Bb5  C5   (classic 4-note pattern)" },
                    SongSection { name: "Verse", chords: "G5  Bb5  C5" },
                ],
                tips: "The ultimate beginner electric riff. Use power chords + distortion.",
            },
            Song {
                title: "Seven Nation Army",
                artist: "The White Stripes",
                year: 2003,
                key: "E",
                difficulty: 1,
                chords_used: &["E", "G", "A", "C"],
                progression: "Iconic single-note riff + power chords",
                sections: &[
                    SongSection { name: "Main Riff", chords: "E (open) – octave pattern" },
                    SongSection { name: "Chorus", chords: "E5  G5  A5  C5" },
                ],
                tips: "Play the riff on the low E string. Extremely satisfying on electric.",
            },
            Song {
                title: "Horse With No Name",
                artist: "America",
                year: 1971,
                key: "Em",
                difficulty: 1,
                chords_used: &["Em", "D6/9 (or D + add9 shape)"],
                progression: "Em – Em9/D (two chord wonder)",
                sections: &[
                    SongSection { name: "Entire Song", chords: "Em          D6add9 / Em9" },
                ],
                tips: "Almost the whole song is two chords. Perfect for beginners who want to play a full song fast.",
            },
            Song {
                title: "Zombie",
                artist: "The Cranberries",
                year: 1994,
                key: "Em",
                difficulty: 2,
                chords_used: &["Em", "C", "G", "D"],
                progression: "Em – C – G – D",
                sections: &[
                    SongSection { name: "Verse / Chorus", chords: "Em  C  G  D" },
                ],
                tips: "Emotional song. Heavy strumming on the chorus. Great for dynamics.",
            },
            Song {
                title: "Creep",
                artist: "Radiohead",
                year: 1992,
                key: "G",
                difficulty: 2,
                chords_used: &["G", "B", "C", "Cm"],
                progression: "G – B – C – Cm",
                sections: &[
                    SongSection { name: "Verse", chords: "G   B   C   Cm" },
                    SongSection { name: "Chorus", chords: "G   B   C   Cm" },
                ],
                tips: "The B and Cm make it special. Practice the B barre carefully.",
            },
            Song {
                title: "Wish You Were Here",
                artist: "Pink Floyd",
                year: 1975,
                key: "G",
                difficulty: 3,
                chords_used: &["G", "Em", "Am", "C", "D"],
                progression: "Beautiful intro riff + G – Em – Am – G – D",
                sections: &[
                    SongSection { name: "Intro Riff", chords: "G  Em  (fingerstyle / picked)" },
                    SongSection { name: "Verse", chords: "C  D  Am  G   D  C  Am  G" },
                ],
                tips: "Learn the intro riff slowly. One of the most beautiful acoustic pieces.",
            },
            Song {
                title: "Nothing Else Matters",
                artist: "Metallica",
                year: 1991,
                key: "Em",
                difficulty: 3,
                chords_used: &["Em", "D", "C", "G", "B"],
                progression: "Em – D – C – G (fingerpicked)",
                sections: &[
                    SongSection { name: "Intro / Verse", chords: "Em  D  C  G   B  Em" },
                    SongSection { name: "Chorus", chords: "C  A  D  G  B  Em" },
                ],
                tips: "Fingerpicking masterpiece. Start slow with the classic opening arpeggio.",
            },
            Song {
                title: "Come As You Are",
                artist: "Nirvana",
                year: 1991,
                key: "Em (drop D optional)",
                difficulty: 2,
                chords_used: &["Em", "D", "G", "A"],
                progression: "Main riff + Em – D – Em – D – A – G",
                sections: &[
                    SongSection { name: "Main Riff", chords: "Em-based sliding riff" },
                    SongSection { name: "Chorus", chords: "A  G  Em  D" },
                ],
                tips: "Grunge classic. The riff is easy once you get the slide motion.",
            },
            Song {
                title: "Hotel California",
                artist: "Eagles",
                year: 1976,
                key: "Bm",
                difficulty: 4,
                chords_used: &["Bm", "F#", "A", "E", "G", "D", "Em"],
                progression: "Bm – F# – A – E – G – D – Em – F#",
                sections: &[
                    SongSection { name: "Verse", chords: "Bm  F#  A  E   G  D  Em  F#" },
                    SongSection { name: "Chorus", chords: "G  D  F#  Bm   G  D  Em  F#" },
                ],
                tips: "More advanced but very rewarding. Learn one section at a time.",
            },
            Song {
                title: "Highway to Hell",
                artist: "AC/DC",
                year: 1979,
                key: "A",
                difficulty: 2,
                chords_used: &["A", "D", "G", "E"],
                progression: "A – G – D – A  /  classic rock power",
                sections: &[
                    SongSection { name: "Intro / Verse", chords: "A  G  D  A" },
                    SongSection { name: "Chorus", chords: "A  D  G  D  A" },
                ],
                tips: "Pure rock energy. Play with attitude and solid downstrokes.",
            },
            Song {
                title: "Back in Black",
                artist: "AC/DC",
                year: 1980,
                key: "E",
                difficulty: 2,
                chords_used: &["E5", "D5", "A5"],
                progression: "Iconic riff + E5 – D5 – A5",
                sections: &[
                    SongSection { name: "Main Riff", chords: "E5 based bluesy rock riff" },
                    SongSection { name: "Chorus", chords: "E5  D5  A5" },
                ],
                tips: "Essential electric guitar song. Focus on the groove and palm muting.",
            },
            Song {
                title: "Dust in the Wind",
                artist: "Kansas",
                year: 1977,
                key: "C",
                difficulty: 3,
                chords_used: &["C", "Am", "G", "Dm", "Em"],
                progression: "Fingerpicked C – Am – G – Dm pattern",
                sections: &[
                    SongSection { name: "Intro / Verse", chords: "C  Am  G  Dm / C  Am  G  Em" },
                ],
                tips: "Beautiful fingerstyle. Practice the Travis-picking pattern slowly.",
            },
            Song {
                title: "House of the Rising Sun",
                artist: "The Animals",
                year: 1964,
                key: "Am",
                difficulty: 2,
                chords_used: &["Am", "C", "D", "F", "E"],
                progression: "Am – C – D – F – Am – E – Am – E",
                sections: &[
                    SongSection { name: "Verse", chords: "Am  C  D  F   Am  E  Am  E" },
                ],
                tips: "Arpeggiate the chords. One of the best songs for learning chord transitions.",
            },
            Song {
                title: "Stand By Me",
                artist: "Ben E. King",
                year: 1961,
                key: "A",
                difficulty: 1,
                chords_used: &["A", "F#m", "D", "E"],
                progression: "A – F#m – D – E (I–vi–IV–V)",
                sections: &[
                    SongSection { name: "Verse / Chorus", chords: "A  F#m  D  E" },
                ],
                tips: "Classic 50s progression. Very beginner friendly and always sounds good.",
            },
            Song {
                title: "Good Riddance (Time of Your Life)",
                artist: "Green Day",
                year: 1997,
                key: "G",
                difficulty: 1,
                chords_used: &["G", "C", "D", "Em"],
                progression: "G – C – D – Em",
                sections: &[
                    SongSection { name: "Verse / Chorus", chords: "G  C  D  Em" },
                    SongSection { name: "Bridge", chords: "Em  D  C  G" },
                ],
                tips: "Strummed acoustic classic. Easy and emotional – great for campfire.",
            },
            Song {
                title: "Blackbird",
                artist: "The Beatles",
                year: 1968,
                key: "G",
                difficulty: 4,
                chords_used: &["G", "Am", "C", "D", "Em", "A"],
                progression: "Complex fingerstyle – G based",
                sections: &[
                    SongSection { name: "Main Theme", chords: "Fingerstyle melody + chords" },
                ],
                tips: "Advanced fingerpicking. One of the most beautiful pieces ever written for guitar.",
            },
            Song {
                title: "Enter Sandman",
                artist: "Metallica",
                year: 1991,
                key: "E (drop D / standard)",
                difficulty: 3,
                chords_used: &["E5", "F5", "G5", "A5"],
                progression: "Heavy riff + power chords",
                sections: &[
                    SongSection { name: "Main Riff", chords: "E5 based palm-muted riff" },
                    SongSection { name: "Chorus", chords: "E5  F5  G5  E5" },
                ],
                tips: "Palm muting is essential. Start clean then add distortion.",
            },
        ];

        Self { songs }
    }
}

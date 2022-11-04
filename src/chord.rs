use std::str::FromStr;

use crate::{Note, note::{Error, Accidental}};

#[derive(Debug, PartialEq)]
struct Chord {
    note: Note,
    symbols: String,
    bass_note: Option<Note>,
}

impl Chord {
    fn new(note: Note, symbols: &str, bass_note: Option<Note>) -> Self {
        Self {
            note,
            symbols: symbols.to_owned(),
            bass_note,
        }
    }
}

impl FromStr for Chord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let natural_char = chars.next().map(|c| c.to_ascii_uppercase());
        let accidental_char = chars.next();

        let note = match Note::from_char_values(natural_char, accidental_char.clone()) {
            Ok(note) => note,
            Err(err) => return Err(err),
        };

        let mut symbols = String::new();

        if note.accidental == Accidental::Natural {
            if let Some(not_accidental) = accidental_char {
                symbols.push(not_accidental)
            }
        }

        let mut building_bass_note = false;
        let mut bass_note_val = String::new();

        for char in chars.into_iter() {
            if building_bass_note {
                bass_note_val.push(char);
            } else if char == '/' {
                building_bass_note = true;
            } else {
                symbols.push(char);
            }
        }

        let bass_note = if bass_note_val == "" {
            None
        } else {
            Chord::from_str(&bass_note_val).map(|v| v.note).ok()
        };

        Ok(Chord::new(note, &symbols, bass_note))
    }
}

impl ToString for Chord {
    fn to_string(&self) -> String {
        use crate::note::Natural::*;
        use Accidental::*;

        let s = if let (B, Natural) = (&self.note.natural, &self.note.accidental) {
            String::from("H")
        } else {
            let natural = match self.note.natural {
                C => "C",
                D => "D",
                E => "E",
                F => "F",
                G => "G",
                A => "A",
                B => "B",
            };

            let accidental = match self.note.accidental {
                Natural => "",
                Flat => "b",
                Sharp => "#",
            };

            String::from(natural) + accidental
        };

        s + &self.symbols
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::note::Accidental::*;
    // use crate::note::Natural::*;
    // use test_case::case;

    // #[case(Chord::new(Natural::A, Accidental::Natural, "", None), "A", "A")]
    // #[case(Chord::new(Natural::G, Accidental::Sharp, "m", None), "g#m", "G#m")]
    // #[case(
    //     Chord::new(Natural::D, Accidental::Flat, "maj7", None),
    //     "Dbmaj7",
    //     "Dbmaj7"
    // )]
    // fn basics(expected_chord: Chord, input_str: &str, pretty_str: &str) {
    //     let chord = Chord::from_str(input_str).unwrap();
    //     assert_eq!(chord, expected_chord);
    //     assert_eq!(chord.to_string(), pretty_str);
    // }
    // #[test]
    // fn bass_note() {
    //     let chord = Chord::from_str("C#m/Db").unwrap();
    //     assert_eq!(
    //         chord,
    //         Chord::new(
    //             Natural::C,
    //             Accidental::Sharp,
    //             "m",
    //             Some(Note {
    //                 natural: Natural::D,
    //                 accidental: Accidental::Flat
    //             })
    //         )
    //     )
    // }
}

// TODO: "Implement bass note"

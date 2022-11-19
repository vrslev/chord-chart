use crate::error::Error;
use crate::note::{Accidental, Note};
use crate::transpose::{Scale, Transpose};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Chord {
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

        let (natural_ch, accidental_ch) = (chars.next(), chars.next());

        let note = Note::parse(natural_ch, accidental_ch)?;

        let mut remaining_symbols = String::new();
        if note.accidental() == &Accidental::Natural {
            if let Some(ch) = accidental_ch {
                // Accidental char wasn't actually an accidental
                remaining_symbols.push(ch)
            }
        }
        remaining_symbols.push_str(chars.as_str());

        let mut building_bass_note = false;
        let mut bass_note_value = String::new();
        let mut symbols = String::new();

        for char in remaining_symbols.chars() {
            if building_bass_note {
                bass_note_value.push(char);
            } else if char == '/' {
                building_bass_note = true;
            } else {
                symbols.push(char);
            }
        }

        let bass_note = if bass_note_value.is_empty() {
            None
        } else {
            let mut chars = bass_note_value.chars();

            match Note::parse(chars.next(), chars.next()) {
                Ok(note) => Some(note),
                Err(Error::NoNatural) => None,
                Err(err) => return Err(err),
            }
        };

        Ok(Chord::new(note, &symbols, bass_note))
    }
}

impl ToString for Chord {
    fn to_string(&self) -> String {
        let note = self.note.to_string();

        if let Some(bass_note) = &self.bass_note {
            note + &self.symbols + "/" + &bass_note.to_string()
        } else {
            note + &self.symbols
        }
    }
}

impl Transpose for Chord {
    fn transpose(&self, semitone_incr: &i32, scale: &Scale) -> Self {
        Self::new(
            Note::transpose(&self.note, semitone_incr, scale),
            &self.symbols,
            self.bass_note
                .as_ref()
                .map(|note| note.transpose(semitone_incr, scale)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Error::*;
    use super::Scale::*;
    use super::*;
    use test_case::case;

    #[case("A", "A")]
    #[case("g#m", "G#m")]
    #[case("Dbmaj7", "Dbmaj7")]
    #[case("C#m/Db", "C#m/Db")]
    #[case("C#m/Dbm", "C#m/Db")]
    #[case("C/db", "C/Db")]
    #[case("C/D", "C/D")]
    #[case("C/", "C")]
    fn basics_ok(input: &str, output: &str) {
        assert_eq!(Chord::from_str(input).unwrap().to_string(), output);
    }

    #[case("w", InvalidNatural('w'))]
    #[case("E#", InvalidNote("E#"))]
    #[case("C/E#", InvalidNote("E#"))]
    #[case("C/W#", InvalidNatural('W'))]
    #[case("A//", InvalidNatural('/'))]
    fn basics_err(input: &str, error: Error) {
        assert_eq!(Chord::from_str(input).unwrap_err(), error);
    }

    #[case("A", 1, Major, "A#")]
    #[case("C#m/D", 1, Major, "Dm/D#")]
    #[case("C#m/D", 12, Major, "C#m/D")]
    #[case("A/D", 4, Minor, "Db/Gb")]
    fn tranpose(input: &str, semitone_incr: i32, scale: Scale, output: &str) {
        assert_eq!(
            Chord::from_str(input)
                .unwrap()
                .transpose(&semitone_incr, &scale)
                .to_string(),
            output
        )
    }
}

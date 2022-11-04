use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Error {
    NoNatural,
    InvalidNatural(char),
    InvalidNote(String),
}

#[derive(Debug, PartialEq)]
enum Natural {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, PartialEq)]
enum Accidental {
    Natural,
    Flat,
    Sharp,
}

#[derive(Debug, PartialEq)]
struct Note {
    natural: Natural,
    accidental: Accidental,
}

impl Note {
    fn from_char_values(
        natural_char: Option<char>,
        accidental_char: Option<char>,
    ) -> Result<Self, Error> {
        use crate::Natural::*;
        use Accidental::*;

        let natural = match natural_char {
            Some(c) => match c.to_ascii_uppercase() {
                'C' => C,
                'D' => D,
                'E' => E,
                'F' => F,
                'G' => G,
                'A' => A,
                'B' => B,
                'H' => {
                    match accidental_char {
                        Some(c) => match c.to_ascii_lowercase() {
                            'b' => return Err(Error::InvalidNote("Hb".into())),
                            '#' => return Err(Error::InvalidNote("H#".into())),
                            _ => (),
                        },
                        _ => (),
                    }

                    return Ok(Self {
                        natural: B,
                        accidental: Natural,
                    });
                }
                _ => return Err(Error::InvalidNatural(c)),
            },
            None => return Err(Error::NoNatural),
        };

        let accidental = match accidental_char {
            Some(c) => match c.to_ascii_lowercase() {
                'b' => Flat,
                '#' => Sharp,
                _ => Natural,
            },
            _ => Natural,
        };

        match (&natural, &accidental) {
            (C, Flat) => return Err(Error::InvalidNote("Cb".into())),
            (E, Sharp) => return Err(Error::InvalidNote("E#".into())),
            (F, Flat) => return Err(Error::InvalidNote("Fb".into())),
            (B, Sharp) => return Err(Error::InvalidNote("B#".into())),
            _ => (),
        }

        Ok(Self {
            natural,
            accidental,
        })
    }
}

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
        use crate::Natural::*;
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
    use super::*;
    use crate::Accidental::*;
    use crate::Natural::*;
    use test_case::case;

    #[case(Some('w'), None, Err(Error::InvalidNatural('w')))]
    #[case(None, None, Err(Error::NoNatural))]
    #[case(Some('a'), None, Ok(Note { natural: A, accidental: Natural}))]
    #[case(Some('A'), Some('b'), Ok(Note { natural: A, accidental: Flat}))]
    #[case(Some('A'), Some('#'), Ok(Note { natural: A, accidental: Sharp}))]
    #[case(Some('A'), Some('w'), Ok(Note { natural: A, accidental: Natural}))]
    #[case(Some('H'), None, Ok(Note { natural: B, accidental: Natural}))]
    #[case(Some('H'), Some('w'), Ok(Note { natural: B, accidental: Natural}))]
    #[case(Some('H'), Some('b'), Err(Error::InvalidNote("Hb".into())))]
    #[case(Some('H'), Some('#'), Err(Error::InvalidNote("H#".into())))]
    #[case(Some('c'), Some('B'), Err(Error::InvalidNote("Cb".into())))]
    #[case(Some('E'), Some('#'), Err(Error::InvalidNote("E#".into())))]
    #[case(Some('F'), Some('b'), Err(Error::InvalidNote("Fb".into())))]
    #[case(Some('B'), Some('#'), Err(Error::InvalidNote("B#".into())))]
    fn note(
        natural_char: Option<char>,
        accidental_char: Option<char>,
        result: Result<Note, Error>,
    ) {
        assert_eq!(
            Note::from_char_values(natural_char, accidental_char),
            result
        );
    }

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

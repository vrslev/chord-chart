#[derive(Debug, PartialEq)]
pub enum Error {
    NoNatural,
    InvalidNatural(char),
    InvalidNote(String),
}

#[derive(Debug, PartialEq)]
pub enum Natural {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, PartialEq)]
pub enum Accidental {
    Natural,
    Flat,
    Sharp,
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub natural: Natural,
    pub accidental: Accidental,
}

impl Note {
    pub fn from_char_values(
        natural_char: Option<char>,
        accidental_char: Option<char>,
    ) -> Result<Self, Error> {
        use self::Natural::*;
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
                        Some(c) => match c {
                            'b' | 'B' => return Err(Error::InvalidNote("Hb".into())),
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

impl ToString for Note {
    fn to_string(&self) -> String {
        use crate::note::Natural::*;
        use Accidental::*;

        if let (B, Natural) = (&self.natural, &self.accidental) {
            "H".into()
        } else {
            let natural = match self.natural {
                C => "C",
                D => "D",
                E => "E",
                F => "F",
                G => "G",
                A => "A",
                B => "B",
            };

            let accidental = match self.accidental {
                Natural => "",
                Flat => "b",
                Sharp => "#",
            };

            String::from(natural) + accidental
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Accidental::*;
    use super::Natural::*;
    use super::*;
    use test_case::case;

    #[case(Some('a'), None, Note { natural: A, accidental: Natural}, "A")]
    #[case(Some('A'), Some('b'), Note { natural: A, accidental: Flat}, "Ab")]
    #[case(Some('A'), Some('#'), Note { natural: A, accidental: Sharp}, "A#")]
    #[case(Some('A'), Some('w'), Note { natural: A, accidental: Natural}, "A")]
    #[case(Some('H'), None, Note { natural: B, accidental: Natural}, "H")]
    #[case(Some('H'), Some('w'), Note { natural: B, accidental: Natural}, "H")]
    fn ok(natural: Option<char>, accidental: Option<char>, note: Note, str: &str) {
        let value = Note::from_char_values(natural, accidental).unwrap();
        assert_eq!(value, note);
        assert_eq!(value.to_string(), str);
    }

    #[case(Some('w'), None, Error::InvalidNatural('w'))]
    #[case(None, None, Error::NoNatural)]
    #[case(Some('H'), Some('b'), Error::InvalidNote("Hb".into()))]
    #[case(Some('H'), Some('#'), Error::InvalidNote("H#".into()))]
    #[case(Some('c'), Some('B'), Error::InvalidNote("Cb".into()))]
    #[case(Some('E'), Some('#'), Error::InvalidNote("E#".into()))]
    #[case(Some('F'), Some('b'), Error::InvalidNote("Fb".into()))]
    #[case(Some('B'), Some('#'), Error::InvalidNote("B#".into()))]
    fn err(natural: Option<char>, accidental: Option<char>, error: Error) {
        let value = Note::from_char_values(natural, accidental).unwrap_err();
        assert_eq!(value, error);
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NoNatural,
    InvalidNatural(char),
    InvalidNote(String),
    InvalidSemitone {
        note_semitone: i32,
        requested_semitone: i32,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Natural {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Accidental {
    Natural = 0,
    Flat = -1,
    Sharp = 1,
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub natural: Natural,
    pub accidental: Accidental,
}

enum Semitone {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
}

pub enum Scale {
    Major,
    Minor,
}

impl Note {
    pub fn new(natural: Natural, accidental: Accidental) -> Self {
        Self {
            natural,
            accidental,
        }
    }

    pub fn from_natural_and_accidental_chars(
        natural_ch: Option<char>,
        accidental_ch: Option<char>,
    ) -> Result<Self, Error> {
        use self::Natural::*;
        use Accidental::*;

        let natural = match natural_ch {
            Some(c) => match c.to_ascii_uppercase() {
                'C' => C,
                'D' => D,
                'E' => E,
                'F' => F,
                'G' => G,
                'A' => A,
                'B' => B,
                'H' => {
                    match accidental_ch {
                        Some(c) => match c {
                            'b' | 'B' => return Err(Error::InvalidNote("Hb".into())),
                            '#' => return Err(Error::InvalidNote("H#".into())),
                            _ => (),
                        },
                        _ => (),
                    }

                    return Ok(Self::new(B, Natural));
                }
                _ => return Err(Error::InvalidNatural(c)),
            },
            None => return Err(Error::NoNatural),
        };

        let accidental = match accidental_ch {
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

        Ok(Self::new(natural, accidental))
    }

    fn from_semitone_and_scale(&self, semitone: Semitone, scale: Scale) -> Self {
        use self::Natural::*;
        use Accidental::*;
        use Semitone::*;

        let (natural, accidental) = match scale {
            Scale::Major => match semitone {
                Zero => (C, Natural),
                One => (C, Sharp),
                Two => (D, Natural),
                Three => (D, Sharp),
                Four => (E, Natural),
                Five => (F, Natural),
                Six => (F, Sharp),
                Seven => (G, Natural),
                Eight => (G, Sharp),
                Nine => (A, Natural),
                Ten => (A, Sharp),
                Eleven => (B, Natural),
            },
            Scale::Minor => match semitone {
                Zero => (C, Natural),
                One => (D, Flat),
                Two => (D, Natural),
                Three => (E, Flat),
                Four => (E, Natural),
                Five => (F, Natural),
                Six => (G, Flat),
                Seven => (G, Natural),
                Eight => (A, Flat),
                Nine => (A, Natural),
                Ten => (B, Flat),
                Eleven => (B, Natural),
            },
        };

        Self::new(natural, accidental)
    }

    pub fn transpose(&self, semitone_incr: i32, scale: Scale) -> Result<Self, Error> {
        use Semitone::*;

        let note_semitone =
            (self.natural.clone() as i32 + self.accidental.clone() as i32).rem_euclid(12);
        let requested_semitone = note_semitone + semitone_incr;

        let semitone = match requested_semitone {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            10 => Ten,
            11 => Eleven,
            _ => {
                return Err(Error::InvalidSemitone {
                    note_semitone,
                    requested_semitone,
                })
            }
        };

        Ok(self.from_semitone_and_scale(semitone, scale))
    }
}

impl ToString for Note {
    fn to_string(&self) -> String {
        use self::Natural::*;
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

    #[case(Some('a'), None, Note::new(A, Natural), "A")]
    #[case(Some('A'), Some('b'), Note::new(A, Flat), "Ab")]
    #[case(Some('A'), Some('#'), Note::new(A, Sharp), "A#")]
    #[case(Some('A'), Some('w'), Note::new(A, Natural), "A")]
    #[case(Some('H'), None, Note::new(B, Natural), "H")]
    #[case(Some('H'), Some('w'), Note::new(B, Natural), "H")]
    fn from_natural_and_accidental_chars_ok(natural: Option<char>, accidental: Option<char>, note: Note, str: &str) {
        let value = Note::from_natural_and_accidental_chars(natural, accidental).unwrap();
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
    fn from_natural_and_accidental_chars_err(natural: Option<char>, accidental: Option<char>, error: Error) {
        let value = Note::from_natural_and_accidental_chars(natural, accidental).unwrap_err();
        assert_eq!(value, error);
    }
}

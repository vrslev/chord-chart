use crate::{
    error::Error,
    transpose::{Scale, Transpose},
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Natural {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Accidental {
    Natural = 0,
    Flat = -1,
    Sharp = 1,
}

impl Accidental {
    pub fn scale(&self) -> Scale {
        match self {
            Self::Flat => Scale::Minor,
            _ => Scale::Major,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Note {
    natural: Natural,
    accidental: Accidental,
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

impl Note {
    fn new(natural: Natural, accidental: Accidental) -> Self {
        Self {
            natural,
            accidental,
        }
    }

    pub fn accidental(&self) -> &Accidental {
        &self.accidental
    }

    pub fn parse(natural_ch: Option<char>, accidental_ch: Option<char>) -> Result<Self, Error> {
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
                    if let Some(c) = accidental_ch {
                        match c {
                            'b' | 'B' => return Err(Error::InvalidNote("Hb")),
                            '#' => return Err(Error::InvalidNote("H#")),
                            _ => (),
                        }
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
            (C, Flat) => return Err(Error::InvalidNote("Cb")),
            (E, Sharp) => return Err(Error::InvalidNote("E#")),
            (F, Flat) => return Err(Error::InvalidNote("Fb")),
            (B, Sharp) => return Err(Error::InvalidNote("B#")),
            _ => (),
        }

        Ok(Self::new(natural, accidental))
    }

    fn from_semitone_and_scale(semitone: &Semitone, scale: &Scale) -> Self {
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

    pub fn get_semitones_diff(&self, note: &Self) -> i32 {
        (note.natural.clone() as i32 + note.accidental.clone() as i32)
            - (self.natural.clone() as i32 + self.accidental.clone() as i32)
    }
}

impl FromStr for Note {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Note::parse(chars.next(), chars.next())
    }
}

impl Transpose for Note {
    fn transpose(&self, semitone_incr: &i32, scale: &Scale) -> Self {
        use Semitone::*;

        let semitone =
            match (self.natural.clone() as i32 + self.accidental.clone() as i32 + semitone_incr)
                .rem_euclid(12)
            {
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
                _ => unreachable!(),
            };

        Self::from_semitone_and_scale(&semitone, scale)
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
    use super::Error::*;
    use super::Natural::*;
    use super::Scale::*;
    use super::*;

    use test_case::case;

    #[case(A, Natural, "a", "A")]
    #[case(A, Flat, "AB", "Ab")]
    #[case(A, Sharp, "A#", "A#")]
    #[case(A, Natural, "Aw", "A")]
    #[case(B, Natural, "H", "H")]
    #[case(B, Natural, "Hw", "H")]
    fn basics_ok(natural: super::Natural, accidental: Accidental, input: &str, output: &str) {
        let value = Note::from_str(input).unwrap();
        assert_eq!(value, Note::new(natural, accidental));
        assert_eq!(value.to_string(), output);
    }

    #[case("w", InvalidNatural('w'))]
    #[case("", NoNatural)]
    #[case("Hb", InvalidNote("Hb"))]
    #[case("H#", InvalidNote("H#"))]
    #[case("cB", InvalidNote("Cb"))]
    #[case("E#", InvalidNote("E#"))]
    #[case("Fb", InvalidNote("Fb"))]
    #[case("B#", InvalidNote("B#"))]
    fn basics_err(input: &str, error: Error) {
        let value = Note::from_str(input).unwrap_err();
        assert_eq!(value, error);
    }

    #[case("A", 12, Major, "A")]
    #[case("C", 1, Major, "C#")]
    #[case("C", -1, Minor, "H")]
    #[case("D", 2, Major, "E")]
    #[case("E", 2, Major, "F#")]
    #[case("E", 2, Minor, "Gb")]
    #[case("A", 11, Major, "G#")]
    #[case("Bb", 1, Minor, "H")]
    fn transpose(input: &str, semitone_incr: i32, scale: Scale, output: &str) {
        let value = Note::from_str(input).unwrap();
        assert_eq!(value.transpose(&semitone_incr, &scale).to_string(), output)
    }

    #[case("C", "C", 0)]
    #[case("C", "C#", 1)]
    #[case("C", "H", 11)]
    #[case("C", "G#", 8)]
    fn get_semitones_diff(note1: &str, note2: &str, output: i32) {
        assert_eq!(
            Note::from_str(note1)
                .unwrap()
                .get_semitones_diff(&Note::from_str(note2).unwrap()),
            output
        )
    }
}

use std::str::FromStr;

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

#[derive(Debug)]
enum Error {
    NoNatural,
    InvalidNatural(char),
    InvalidNote(String),
}

#[derive(Debug, PartialEq)]
struct Chord {
    natural: Natural,
    accidental: Accidental,
    symbols: String,
}

impl Chord {
    fn new(natural: Natural, accidental: Accidental, symbols: &str) -> Self {
        Self {
            natural,
            accidental,
            symbols: symbols.to_owned(),
        }
    }
}

impl FromStr for Chord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Natural::*;
        use Accidental::*;

        let mut chars = s.chars();

        let natural = match chars.next() {
            Some(c) => match c.to_ascii_uppercase() {
                'C' => C,
                'D' => D,
                'E' => E,
                'F' => F,
                'G' => G,
                'A' => A,
                'B' => B,
                _ => return Err(Error::InvalidNatural(c)),
            },
            None => return Err(Error::NoNatural),
        };

        let mut symbols = String::new();

        let accidental = match chars.next() {
            Some('b') => Flat,
            Some('#') => Sharp,
            Some(c) => {
                symbols.push(c);
                Natural
            }
            None => Natural,
        };

        match (&natural, &accidental) {
            (C, Flat) => return Err(Error::InvalidNote("Cb".into())),
            (E, Sharp) => return Err(Error::InvalidNote("E#".into())),
            (F, Flat) => return Err(Error::InvalidNote("Fb".into())),
            (B, Sharp) => return Err(Error::InvalidNote("B#".into())),
            _ => (),
        }

        symbols.push_str(chars.as_str());

        Ok(Chord::new(natural, accidental, &symbols))
    }
}

impl ToString for Chord {
    fn to_string(&self) -> String {
        use crate::Natural::*;
        use Accidental::*;

        let s = if let (B, Natural) = (&self.natural, &self.accidental) {
            String::from("H")
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
        };

        s + &self.symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::case;

    #[case(Chord::new(Natural::A, Accidental::Natural, ""), "A", "A")]
    #[case(Chord::new(Natural::G, Accidental::Sharp, "m"), "g#m", "G#m")]
    #[case(Chord::new(Natural::D, Accidental::Flat, "maj7"), "Dbmaj7", "Dbmaj7")]
    fn basics(expected_chord: Chord, input_str: &str, pretty_str: &str) {
        let chord = Chord::from_str(input_str).unwrap();
        assert_eq!(chord, expected_chord);
        assert_eq!(chord.to_string(), pretty_str);
    }
}

// TODO: "Implement bass note"

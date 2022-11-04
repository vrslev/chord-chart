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

#[cfg(test)]
mod tests {
    use super::Accidental::*;
    use super::Natural::*;
    use super::*;
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
}

use crate::chord::Chord;
use crate::error::Error;
use crate::transpose::{Scale, Transpose};
use std::str::FromStr;

#[derive(Debug)]
pub struct Bar(Vec<Chord>);

impl FromStr for Bar {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = Vec::new();
        for s in s.split_whitespace() {
            vec.push(Chord::from_str(s)?);
        }
        Ok(Bar(vec))
    }
}

impl ToString for Bar {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(Chord::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Transpose for Bar {
    fn transpose(&self, semitone_incr: &i32, scale: &Scale) -> Self {
        Bar(self
            .0
            .iter()
            .map(|c| c.transpose(semitone_incr, scale))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Scale;
    use super::*;
    use test_case::case;

    #[case("C Dm/F#", "C Dm/F#")]
    #[case("C dm/f#  D", "C Dm/F# D")]
    fn basics(input: &str, output: &str) {
        assert_eq!(Bar::from_str(input).unwrap().to_string(), output);
    }

    #[test]
    fn tranpose() {
        assert_eq!(
            Bar::from_str("A/D C#m/D")
                .unwrap()
                .transpose(&4, &Scale::Minor)
                .to_string(),
            "Db/Gb Fm/Gb"
        )
    }
}

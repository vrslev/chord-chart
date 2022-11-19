use std::str::FromStr;

use crate::{
    bar::Bar,
    error::Error,
    transpose::{Scale, Transpose},
};

#[derive(Debug)]
pub struct BarLine(Vec<Bar>);

impl FromStr for BarLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(BarLine(Vec::new()));
        }

        let mut chars = trimmed.chars();
        if chars.next() != Some('|') {
            return Err(Error::BarLineShouldStartWithStripe(s.into()));
        }
        if chars.next_back() != Some('|') {
            return Err(Error::BarLineShouldEndWithStripe(s.into()));
        }

        let mut vec = Vec::new();
        for bar in chars.as_str().split('|') {
            vec.push(Bar::from_str(bar)?);
        }
        Ok(BarLine(vec))
    }
}

impl ToString for BarLine {
    fn to_string(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        let joined_bars = self
            .0
            .iter()
            .map(Bar::to_string)
            .collect::<Vec<String>>()
            .join(" | ");

        format!("| {joined_bars} |")
    }
}

impl Transpose for BarLine {
    fn transpose(&self, semitone_incr: &i32, scale: &Scale) -> Self {
        BarLine(
            self.0
                .iter()
                .map(|b| b.transpose(semitone_incr, scale))
                .collect(),
        )
    }
}

impl BarLine {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Scale;
    use super::*;
    use test_case::case;

    #[test]
    fn basics_ok() {
        let test =
            |input, output: &str| assert_eq!(BarLine::from_str(input).unwrap().to_string(), output);
        test("|C Dm/F# | C Dm/F# |", "| C Dm/F# | C Dm/F# |");
        test(" |  C Dm/F#  | ", "| C Dm/F# |");
        test("||", "|  |");
        test("", "");
    }

    #[case("C", Error::BarLineShouldStartWithStripe("C".into()))]
    #[case("|C", Error::BarLineShouldEndWithStripe("|C".into()))]
    #[case("|W|", Error::InvalidNatural('W'))]
    fn basics_err(input: &str, error: Error) {
        assert_eq!(BarLine::from_str(input).unwrap_err(), error);
    }

    #[test]
    fn tranpose() {
        assert_eq!(
            BarLine::from_str("| A/D C#m/D | A/D C#m/D |")
                .unwrap()
                .transpose(&4, &Scale::Minor)
                .to_string(),
            "| Db/Gb Fm/Gb | Db/Gb Fm/Gb |"
        )
    }
}

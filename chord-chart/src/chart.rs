use std::str::FromStr;

use crate::{bar_line::BarLine, error::Error, transpose::{Transpose, Scale}};

pub struct Chart(Vec<BarLine>);

impl FromStr for Chart {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = Vec::new();
        for line in s.split("\n").into_iter() {
            vec.push(BarLine::from_str(line)?);
        }
        Ok(Chart(vec))
    }
}

impl ToString for Chart {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .filter(|l| !l.is_empty())
            .map(BarLine::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Transpose for Chart {
    fn transpose(&self, semitone_incr: &i32, scale: &Scale) -> Self {
        Chart(
            self.0
                .iter()
                .map(|l| l.transpose(semitone_incr, scale))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Scale;

    #[test]
    fn basics() {
        let input = r#"

        | G/C | Bm | A |
        | D |
        | A |

        | Bm | A | G |
        "#;
        let output = "| G/C | Hm | A |\n| D |\n| A |\n| Hm | A | G |";

        assert_eq!(Chart::from_str(input).unwrap().to_string(), output);
    }

    #[test]
    fn tranpose() {
        let input = r#"

        | G/C | Bm | A |
        | D |
        | A |

        | Bm | A | G |
        "#;
        let output = "| H/E | Ebm | Db |\n| Gb |\n| Db |\n| Ebm | Db | H |";

        assert_eq!(
            Chart::from_str(input)
                .unwrap()
                .transpose(&4, &Scale::Minor)
                .to_string(),
            output
        )
    }
}

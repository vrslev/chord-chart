use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoNatural,
    InvalidNatural(char),
    InvalidNote(&'static str),
    BarLineShouldStartWithStripe(String),
    BarLineShouldEndWithStripe(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        let str: String = match &self {
            NoNatural => "no natural".into(),
            InvalidNatural(natural) => format!("invalid natural: {natural}"),
            InvalidNote(note) => format!("invalid note: {note}"),
            BarLineShouldStartWithStripe(line) => {
                format!("bar line should start with stripe: {line}")
            }
            BarLineShouldEndWithStripe(line) => format!("bar line should end with stripe: {line}"),
        };

        write!(f, "{}", str)
    }
}

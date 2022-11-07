#[derive(Debug, PartialEq)]
pub enum Error {
    NoNatural,
    InvalidNatural(char),
    InvalidNote(&'static str),
    BarLineShouldStartWithStripe(String),
    BarLineShouldEndWithStripe(String),
}

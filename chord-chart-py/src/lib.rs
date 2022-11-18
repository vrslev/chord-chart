use std::str::FromStr;

use chord_chart::Transpose;
use pyo3::{create_exception, exceptions::PyException, prelude::*};

create_exception!(_chord_chart, ValidationError, PyException);

#[derive(Debug)]
struct Error(chord_chart::Error);

impl From<chord_chart::Error> for Error {
    fn from(error: chord_chart::Error) -> Self {
        Self(error)
    }
}

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        use chord_chart::Error::*;

        let str: String = match &error.0 {
            NoNatural => "no natural".into(),
            InvalidNatural(natural) => format!("invalid natural: {natural}"),
            InvalidNote(note) => format!("invalid note: {note}"),
            BarLineShouldStartWithStripe(line) => {
                format!("bar line should start with stripe: {line}")
            }
            BarLineShouldEndWithStripe(line) => format!("bar line should end with stripe: {line}"),
        };

        ValidationError::new_err(str)
    }
}

#[pyfunction]
fn validate_chart(chart: &str) -> Result<String, Error> {
    Ok(chord_chart::Chart::from_str(chart)?.to_string())
}

#[pyfunction]
fn transpose_chart(chart: &str, current_key: &str, new_key: &str) -> Result<String, Error> {
    let current_key_value = chord_chart::Note::from_str(current_key)?;
    let new_key_value = chord_chart::Note::from_str(new_key)?;
    Ok(chord_chart::Chart::from_str(chart)?
        .transpose(
            &current_key_value.get_semitones_diff(&new_key_value),
            &new_key_value.accidental().scale(),
        )
        .to_string())
}

#[pymodule]
#[pyo3(name = "_chord_chart")]
fn module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_chart, m)?)?;
    m.add_function(wrap_pyfunction!(transpose_chart, m)?)?;
    m.add("ValidationError", py.get_type::<ValidationError>())?;
    Ok(())
}

use std::{fmt, str::FromStr};

use chord_chart::Transpose;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
struct Error(chord_chart::Error);

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<chord_chart::Error> for Error {
    fn from(error: chord_chart::Error) -> Self {
        Self(error)
    }
}

#[wasm_bindgen(js_name = validateChart)]
pub fn validate_chart(chart: &str) -> Result<String, JsError> {
    fn _validate(chart: &str) -> Result<String, Error> {
        Ok(chord_chart::Chart::from_str(chart)?.to_string())
    }
    Ok(_validate(chart)?)
}

#[wasm_bindgen(js_name = tranposeChart)]
pub fn tranpose_chart(chart: &str, current_key: &str, new_key: &str) -> Result<String, JsError> {
    fn _transpose(chart: &str, current_key: &str, new_key: &str) -> Result<String, Error> {
        let current_key_value = chord_chart::Note::from_str(current_key)?;
        let new_key_value = chord_chart::Note::from_str(new_key)?;
        Ok(chord_chart::Chart::from_str(chart)?
            .transpose(
                &current_key_value.get_semitones_diff(&new_key_value),
                &new_key_value.accidental().scale(),
            )
            .to_string())
    }
    Ok(_transpose(chart, current_key, new_key)?)
}

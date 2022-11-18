use std::str::FromStr;

use chord_chart::Transpose;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@index")]
extern "C" {
    #[wasm_bindgen]
    type ValidationError;

    #[wasm_bindgen(constructor)]
    fn new(type_: &str, value: Option<&str>) -> ValidationError;
}

impl From<chord_chart::Error> for ValidationError {
    fn from(error: chord_chart::Error) -> Self {
        use chord_chart::Error::*;
        match &error {
            NoNatural => Self::new("NoNatural", None),
            InvalidNatural(natural) => Self::new("InvalidNatural", Some(&natural.to_string())),
            InvalidNote(note) => Self::new("InvalidNote", Some(note)),
            BarLineShouldStartWithStripe(line) => {
                Self::new("BarLineShouldStartWithStripe", Some(line))
            }
            BarLineShouldEndWithStripe(line) => Self::new("BarLineShouldEndWithStripe", Some(line)),
        }
    }
}

#[wasm_bindgen(js_name = validateChart)]
pub fn validate_chart(chart: &str) -> Result<String, JsValue> {
    fn _validate(chart: &str) -> Result<String, ValidationError> {
        Ok(chord_chart::Chart::from_str(chart)?.to_string())
    }
    Ok(_validate(chart)?)
}
#[wasm_bindgen(js_name = transposeChart)]
pub fn transpose_chart(chart: &str, current_key: &str, new_key: &str) -> Result<String, JsValue> {
    fn _transpose(
        chart: &str,
        current_key: &str,
        new_key: &str,
    ) -> Result<String, ValidationError> {
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

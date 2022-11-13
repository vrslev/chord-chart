use std::str::FromStr;

use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = validateChart)]
pub fn validate_chart(value: &str) -> Result<String, JsError> {
    set_panic_hook();

    let chart = match chord_chart::Chart::from_str(value) {
        Ok(chart) => chart,
        Err(err) => return Err(JsError::new(&err.to_string())),
    };
    Ok(chart.to_string())
}

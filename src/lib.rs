mod convert;
mod types;

use convert::{deocde, encode};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn encode_integer(int: &str, size: u8, signed: bool) -> Result<String, JsValue> {
    let size = types::Size::try_from(size)?;

    let result = encode(int, size, signed).map_err(|e| e.to_string())?;

    Ok(result)
}

#[wasm_bindgen]
pub fn decode_string(str: &str, size: u8, signed: bool) -> Result<String, JsValue> {
    let size = types::Size::try_from(size)?;

    let result = deocde(str, size, signed).map_err(|e| e.to_string())?;

    Ok(result)
}

#[wasm_bindgen]
pub fn random_integer(size: u8, signed: bool) -> Result<String, JsValue> {
    let size = types::Size::try_from(size)?;

    let result = convert::random_integer(size, signed);

    Ok(result)
}

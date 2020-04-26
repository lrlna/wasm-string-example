use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn return_string() -> JsValue {
  return JsValue::from_str("A new JsValue".into())
}

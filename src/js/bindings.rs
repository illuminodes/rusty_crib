use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(module = "/src/js/utils.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn clipboardCopy(t: &str) -> Result<(), JsValue>;
}

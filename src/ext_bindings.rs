use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[cfg(feature = "tauri")]
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "console"])]
    pub fn log(stuff: JsValue);

    #[wasm_bindgen(js_namespace = ["window", "Date"])]
    pub fn now() -> f64;
}
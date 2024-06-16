use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = solana)]
    pub static SOLANA: Solana;

    #[wasm_bindgen]
    pub type Solana;

    #[wasm_bindgen(method, js_name = connect)]
    pub fn connect(this: &Solana, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Solana) -> Promise;

    #[wasm_bindgen(method, getter)]
    pub fn publicKey(this: &Solana) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn signTransaction(this: &Solana, transaction: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn signAllTransactions(this: &Solana, transactions: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn signAndSendTransaction(this: &Solana, transaction: JsValue) -> Promise;
}

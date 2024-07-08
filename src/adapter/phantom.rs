use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = solana)]
    pub static SOLANA: Solana;

    #[wasm_bindgen]
    pub type Solana;

    #[wasm_bindgen(method, js_name = accountChanged)]
    pub fn account_changed(this: &Solana, listener: &JsValue);

    #[wasm_bindgen(method)]
    pub fn connect(this: &Solana, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Solana) -> Promise;

    #[wasm_bindgen(method, getter)]
    pub fn publicKey(this: &Solana) -> JsValue;

    #[wasm_bindgen(method, js_name = handleNotification)]
    pub fn handle_notification(this: &Solana, notification: &JsValue);

    #[wasm_bindgen(method, js_name = removeAllListeners)]
    pub fn remove_all_listeners(this: &Solana, event: &JsValue);

    #[wasm_bindgen(method)]
    pub fn request(this: &Solana, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn signAllTransactions(this: &Solana, transactions: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signAndSendTransaction)]
    pub fn sign_and_send_transaction(
        this: &Solana,
        transaction: &JsValue,
        options: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = signAndSendAllTransactions)]
    pub fn sign_and_send_all_transactions(
        this: &Solana,
        transactions: &JsValue,
        options: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = signIn)]
    pub fn sign_in(this: &Solana, options: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signMessage)]
    pub fn sign_message(this: &Solana, message: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signTransaction)]
    pub fn sign_transaction(this: &Solana, transaction: &JsValue) -> Promise;
}

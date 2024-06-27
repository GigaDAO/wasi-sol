use js_sys::{Array, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = solflare)]
    pub static SOLFLARE: Solflare;

    #[wasm_bindgen(extends = Object)]
    pub type Solflare;

    #[wasm_bindgen(method, getter, js_name = isSolflare)]
    pub fn is_solflare(this: &Solflare) -> bool;

    #[wasm_bindgen(method, getter, js_name = isConnected)]
    pub fn is_connected(this: &Solflare) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn publicKey(this: &Solflare) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = priorityFeesSupported)]
    pub fn priority_fees_supported(this: &Solflare) -> bool;

    #[wasm_bindgen(method, getter, js_name = autoApprove)]
    pub fn auto_approve(this: &Solflare) -> bool;

    #[wasm_bindgen(method)]
    pub fn request(this: &Solflare, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn connect(this: &Solflare, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Solflare) -> Promise;

    #[wasm_bindgen(method, js_name = signTransaction)]
    pub fn sign_transaction(this: &Solflare, transaction: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signMessage)]
    pub fn sign_message(this: &Solflare, message: &JsValue, options: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signAllTransactions)]
    pub fn sign_all_transactions(this: &Solflare, transactions: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signAndSendTransaction)]
    pub fn sign_and_send_transaction(
        this: &Solflare,
        transaction: &JsValue,
        options: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method)]
    pub fn on(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = addListener)]
    pub fn add_listener(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method)]
    pub fn once(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method)]
    pub fn off(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = removeEventListener)]
    pub fn remove_event_listener(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = removeListener)]
    pub fn remove_listener(this: &Solflare, event: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = eventNames)]
    pub fn event_names(this: &Solflare) -> Array;

    #[wasm_bindgen(method, js_name = listenerCount)]
    pub fn listener_count(this: &Solflare, event: &JsValue) -> u32;

    #[wasm_bindgen(method, js_name = listeners)]
    pub fn listeners(this: &Solflare, event: &JsValue) -> Array;

    #[wasm_bindgen(method, js_name = removeAllListeners)]
    pub fn remove_all_listeners(this: &Solflare, event: &JsValue);
}

use js_sys::{Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = xnft, js_name = solana)]
    pub static BACKPACK: Backpack;

    #[wasm_bindgen(js_namespace = window, js_name = xnft)]
    pub static XNFT: Backpack;

    #[wasm_bindgen(extends = Object)]
    pub type Backpack;

    #[wasm_bindgen(method)]
    pub fn connect(this: &Backpack, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Backpack) -> Promise;

    #[wasm_bindgen(method, js_name = openXnft)]
    pub fn open_xnft(this: &Backpack, xnft: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = _backpackGetAccounts)]
    pub fn backpack_get_accounts(this: &Backpack) -> Promise;

    #[wasm_bindgen(method, js_name = signIn)]
    pub fn sign_in(this: &Backpack, options: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = sendAndConfirm)]
    pub fn send_and_confirm(
        this: &Backpack,
        tx: &JsValue,
        signers: &JsValue,
        options: &JsValue,
        custom_connection: &JsValue,
        uuid: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = signAndSendTransaction)]
    pub fn sign_and_send_transaction(this: &Backpack, tx: &JsValue, options: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn send(
        this: &Backpack,
        tx: &JsValue,
        signers: &JsValue,
        options: &JsValue,
        custom_connection: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = sendAll)]
    pub fn send_all(
        this: &Backpack,
        txs: &JsValue,
        signers: &JsValue,
        options: &JsValue,
        custom_connection: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method)]
    pub fn simulate(
        this: &Backpack,
        tx: &JsValue,
        signers: &JsValue,
        commitment: &JsValue,
        custom_connection: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = signTransaction)]
    pub fn sign_transaction(
        this: &Backpack,
        tx: &JsValue,
        public_key: &JsValue,
        custom_connection: &JsValue,
        uuid: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = signAllTransactions)]
    pub fn sign_all_transactions(
        this: &Backpack,
        txs: &JsValue,
        public_key: &JsValue,
        custom_connection: &JsValue,
        uuid: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = prepareSolanaOffchainMessage)]
    pub fn prepare_solana_offchain_message(this: &Backpack, message: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = signMessage)]
    pub fn sign_message(
        this: &Backpack,
        message: &JsValue,
        public_key: &JsValue,
        uuid: &JsValue,
    ) -> Promise;

    #[wasm_bindgen(method, getter, js_name = isBackpack)]
    pub fn is_backpack(this: &Backpack) -> bool;

    #[wasm_bindgen(method, getter, js_name = isConnected)]
    pub fn is_connected(this: &Backpack) -> bool;

    #[wasm_bindgen(method, getter, js_name = isXnft)]
    pub fn is_xnft(this: &Backpack) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn publicKey(this: &Backpack) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn connection(this: &Backpack) -> JsValue;
}

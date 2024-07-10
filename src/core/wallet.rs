#![allow(deprecated)]

use std::str::FromStr;

use anyhow::Result;
use emitter_rs::EventEmitter;
use js_sys::{Promise, Uint8Array};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use solana_client_wasm::WasmClient as RpcClient;
use solana_sdk::{bs58, pubkey::Pubkey, signature::Signature, transaction::Transaction};

use crate::{
    adapter::{
        backpack::{BACKPACK, XNFT},
        phantom::SOLANA,
        solflare::SOLFLARE,
    },
    core::{
        error::WalletError,
        response::{JsSignatureObject, JsSignatureResponse, SignaturesObject},
        traits::{WalletAdapter, WalletAdapterEvents},
        transaction::TransactionOrVersionedTransaction,
    },
};

use gloo_storage::{LocalStorage, Storage};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Wallet {
    #[default]
    Phantom,
    Solflare,
    Backpack,
}

impl From<Wallet> for BaseWalletAdapter {
    fn from(val: Wallet) -> Self {
        match val {
            Wallet::Phantom => BaseWalletAdapter::new(
                Wallet::Phantom,
                "https://phantom.app",
                "images/phantom_logo.png",
            ),
            Wallet::Solflare => BaseWalletAdapter::new(
                Wallet::Solflare,
                "https://solflare.com",
                "images/solflare_logo.png",
            ),
            Wallet::Backpack => BaseWalletAdapter::new(
                Wallet::Backpack,
                "https://backpack.app",
                "images/backpack_logo.png",
            ),
        }
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum WalletReadyState {
    Installed,
    #[default]
    NotDetected,
    Loadable,
    Unsupported,
}

#[derive(Default, Clone, PartialEq)]
pub struct BaseWalletAdapter {
    name: Wallet,
    url: String,
    icon: String,
    ready_state: WalletReadyState,
    public_key: Option<Pubkey>,
    connecting: bool,
    pub emitter: EventEmitter,
}

impl BaseWalletAdapter {
    pub fn new(name: Wallet, url: &str, icon: &str) -> Self {
        let ready_state = if cfg!(target_arch = "wasm32") {
            WalletReadyState::Unsupported
        } else {
            WalletReadyState::NotDetected
        };

        BaseWalletAdapter {
            ready_state,
            name,
            url: url.to_string(),
            icon: icon.to_string(),
            public_key: None,
            connecting: false,
            emitter: EventEmitter::new(),
        }
    }
}

impl WalletAdapter for BaseWalletAdapter {
    fn name(&self) -> String {
        format!("{:?}", self.name).to_string()
    }

    fn url(&self) -> String {
        self.url.to_string()
    }

    fn icon(&self) -> String {
        self.icon.to_string()
    }

    fn ready_state(&self) -> WalletReadyState {
        self.ready_state.clone()
    }

    fn public_key(&self) -> Option<Pubkey> {
        self.public_key
    }

    fn connecting(&self) -> bool {
        self.connecting
    }

    async fn auto_connect(&mut self) -> Result<bool, WalletError> {
        self.connect().await
    }

    async fn connect(&mut self) -> Result<bool, WalletError> {
        info!("Connecting to wallet...");

        if self.connecting {
            self.emit_error(WalletError::WalletConnectionError);
            return Err(WalletError::WalletConnectionError);
        }

        self.connecting = true;

        LocalStorage::set("walletName", &self.name()).ok();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(
            &options,
            &serde_wasm_bindgen::to_value("onlyIfTrusted").unwrap(),
            &serde_wasm_bindgen::to_value(&true).unwrap(),
        )
        .unwrap();

        let promise: Option<Promise> = match self.name {
            Wallet::Phantom if !SOLANA.is_undefined() => Some(SOLANA.sign_in(&options)),
            Wallet::Solflare if !SOLFLARE.is_undefined() => Some(SOLFLARE.connect(&options)),
            Wallet::Backpack if !XNFT.is_undefined() => Some(BACKPACK.sign_in(&options)),
            Wallet::Phantom | Wallet::Solflare | Wallet::Backpack => None,
        };

        if promise.is_some() {
            let result = JsFuture::from(promise.unwrap()).await;

            match result {
                Ok(_response) => {
                    // Todo use response to get pubkey
                    // let response: MessageObject = serde_wasm_bindgen::from_value(response).unwrap();
                    info!("Wallet connected");

                    let key: JsValue = match self.name {
                        Wallet::Phantom => SOLANA.publicKey(),
                        Wallet::Solflare => SOLFLARE.publicKey(),
                        Wallet::Backpack => BACKPACK.publicKey(),
                    };

                    if key.is_undefined() {
                        info!("Public key is undefined");
                    } else {
                        let key_str: String = JsValue::into_serde(&key).unwrap();

                        let public_key = Pubkey::from_str(&key_str).unwrap();
                        info!("Connected to wallet with public key: {:?}", public_key);
                        LocalStorage::set("pubKey", &public_key).ok();
                        self.public_key = Some(public_key);
                        self.ready_state = WalletReadyState::Installed;
                        self.emit_connect(public_key);

                        self.connecting = false;
                    }
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {:?}", err);
                }
            }
        } else {
            let window = window().expect("no global `window` exists");
            window
                .open_with_url(&self.url)
                .expect("failed to open a new tab");
        }

        Ok(!self.connecting)
    }

    async fn disconnect(&mut self) -> Result<bool, WalletError> {
        info!("Disconnecting from wallet...");
        let mut confirmed = false;

        let public_key: Pubkey = LocalStorage::get("pubKey").unwrap();
        let wallet_name: Wallet = LocalStorage::get("walletName").unwrap();

        if !public_key.to_string().is_empty() {
            self.public_key = Some(public_key);
            self.name = wallet_name;
        }

        if self.public_key.is_none() {
            self.emit_error(WalletError::WalletDisconnectedError);
            return Err(WalletError::WalletDisconnectedError);
        }

        self.public_key = None;
        self.ready_state = WalletReadyState::NotDetected;
        self.emit_disconnect();
        let promise: Promise = match self.name {
            Wallet::Phantom => SOLANA.disconnect(),
            Wallet::Solflare => SOLFLARE.disconnect(),
            Wallet::Backpack => BACKPACK.disconnect(),
        };
        let result = JsFuture::from(promise).await;

        match result {
            Ok(_) => {
                confirmed = true;
                info!("Disconnected from wallet");
            }
            Err(err) => {
                log::error!("Failed to disconnect wallet: {:?}", err);
            }
        }

        Ok(confirmed)
    }

    async fn send_transaction(
        &mut self,
        client: Arc<RpcClient>,
        transaction: TransactionOrVersionedTransaction,
    ) -> Result<Signature, WalletError> {
        info!("Sending transaction...");

        if self.public_key.is_none() {
            self.emit_error(WalletError::WalletNotConnectedError);
            return Err(WalletError::WalletNotConnectedError);
        }
        let signature = match transaction {
            TransactionOrVersionedTransaction::Transaction(tx) => client
                .send_and_confirm_transaction(&tx)
                .await
                .map_err(|_| WalletError::WalletSendTransactionError)?,
            TransactionOrVersionedTransaction::VersionedTransaction(_vtx) => {
                // TODO: Add support
                // client
                //     .async_send_versioned_transaction(&vtx)
                //     .await
                //     .map_err(|_| WalletError::WalletSendTransactionError)?
                Signature::default()
            }
        };

        self.emit_transaction_sent(signature);
        info!("Transaction sent: {}", signature);
        Ok(signature)
    }

    async fn sign_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, WalletError> {
        info!("Signing transaction...");

        if self.public_key.is_none() {
            self.emit_error(WalletError::WalletNotConnectedError);
            return Err(WalletError::WalletNotConnectedError);
        }

        let transaction_bytes = bincode::serialize(&transaction)
            .map_err(|_| WalletError::WalletSignTransactionError)?;

        let transaction_js_array = Uint8Array::from(&transaction_bytes[..]);

        let bs58_tx = bs58::encode(transaction_bytes).into_string();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(
            &options,
            &serde_wasm_bindgen::to_value("method").unwrap(),
            &serde_wasm_bindgen::to_value("signTransaction").unwrap(),
        )
        .expect("Failed to set method in options");

        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &serde_wasm_bindgen::to_value("message").unwrap(),
            &serde_wasm_bindgen::to_value(&bs58_tx).unwrap(),
        )
        .expect("Failed to set message in params");

        js_sys::Reflect::set(
            &options,
            &serde_wasm_bindgen::to_value("params").unwrap(),
            &JsValue::from(&params),
        )
        .expect("Failed to set params in options");

        let promise: Promise = match self.name {
            Wallet::Phantom => SOLANA.request(&options),
            Wallet::Solflare => SOLFLARE.request(&options),
            Wallet::Backpack => BACKPACK.sign_transaction(
                &transaction_js_array,
                &JsValue::from(self.public_key.unwrap()),
                &JsValue::from(""),
                &JsValue::from("uuid"),
            ),
        };

        let result = JsFuture::from(promise).await;

        match result {
            Ok(json_str) => {
                let deserialized: SignaturesObject = JsValue::into_serde(&json_str).unwrap();

                let signature_map = &deserialized.signatures[0].signature;
                let mut signature_bytes = [0u8; 64];

                for (key, value) in signature_map.iter() {
                    let index: usize = key.parse().unwrap();
                    signature_bytes[index] = value.as_u64().unwrap() as u8;
                }

                let signature = Signature::new(&signature_bytes);
                info!("Got signature: {:?}", signature);
                Ok(signature)
            }
            Err(err) => {
                log::error!("Failed to sign transaction: {:?}", err);
                Err(WalletError::WalletSignTransactionError)
            }
        }
    }

    async fn sign_send_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<Signature, WalletError> {
        info!("Signing and sending transaction...");

        if self.public_key.is_none() {
            self.emit_error(WalletError::WalletNotConnectedError);
            return Err(WalletError::WalletNotConnectedError);
        }

        let transaction_bytes = bincode::serialize(&transaction)
            .map_err(|_| WalletError::WalletSignTransactionError)?;

        let transaction_js_array = Uint8Array::from(&transaction_bytes[..]);

        let bs58_tx = bs58::encode(transaction_bytes).into_string();

        let options = js_sys::Object::new();
        js_sys::Reflect::set(
            &options,
            &serde_wasm_bindgen::to_value("method").unwrap(),
            &serde_wasm_bindgen::to_value("signAndSendTransaction").unwrap(),
        )
        .expect("Failed to set method in options");

        let params = js_sys::Object::new();
        js_sys::Reflect::set(
            &params,
            &serde_wasm_bindgen::to_value("message").unwrap(),
            &serde_wasm_bindgen::to_value(&bs58_tx).unwrap(),
        )
        .expect("Failed to set message in params");

        js_sys::Reflect::set(
            &options,
            &serde_wasm_bindgen::to_value("params").unwrap(),
            &JsValue::from(&params),
        )
        .expect("Failed to set params in options");

        let promise: Promise = match self.name {
            Wallet::Phantom => SOLANA.request(&options),
            Wallet::Solflare => SOLFLARE.request(&options),
            Wallet::Backpack => BACKPACK.sign_and_send_transaction(&transaction_js_array, &options),
        };

        let result = JsFuture::from(promise).await;

        match result {
            Ok(json_str) => {
                let deserialized: JsSignatureObject = JsValue::into_serde(&json_str).unwrap();

                let signature = Signature::from_str(&deserialized.signature).unwrap();
                info!("Got signature: {:?}", signature);

                Ok(signature)
            }
            Err(err) => {
                log::error!("Failed to sign transaction: {:?}", err);
                Err(WalletError::WalletSignTransactionError)
            }
        }
    }

    async fn sign_message(&mut self, message: &str) -> Result<Signature, WalletError> {
        info!("Signing transaction...");

        if self.public_key.is_none() {
            return Err(WalletError::WalletNotConnectedError);
        }

        let message_bytes =
            bincode::serialize(&message).map_err(|_| WalletError::WalletSignTransactionError)?;

        let message_js_array = Uint8Array::from(&message_bytes[..]);

        let promise: Promise = match self.name {
            Wallet::Phantom => SOLANA.sign_message(&message_js_array),
            Wallet::Solflare => SOLFLARE.sign_message(&message_js_array),
            Wallet::Backpack => BACKPACK.sign_message(
                &message_js_array,
                &JsValue::from(self.public_key.unwrap()),
                &JsValue::from("uuid"),
            ),
        };

        let result = JsFuture::from(promise).await;

        match result {
            Ok(json_str) => {
                let sig_obj: JsSignatureResponse = JsValue::into_serde(&json_str).unwrap();
                let data = sig_obj.signature.data;

                let signature = Signature::new(&data);
                info!("Message signed: {:?}", signature);

                Ok(signature)
            }
            Err(err) => {
                log::error!("Failed to sign transaction: {:?}", err);
                Err(WalletError::WalletSignTransactionError)
            }
        }
    }
}

impl WalletAdapterEvents for BaseWalletAdapter {
    fn emit_connect(&mut self, public_key: Pubkey) {
        self.emitter.emit("connect", public_key);
    }

    fn emit_disconnect(&mut self) {
        self.emitter.emit("disconnect", ());
    }

    fn emit_error(&mut self, error: WalletError) {
        self.emitter.emit("error", error);
    }

    fn ready_state_change(&mut self, ready_state: WalletReadyState) {
        self.emitter.emit("ready_state_change", ready_state);
    }

    fn emit_transaction_sent(&mut self, signature: Signature) {
        self.emitter.emit("transaction_sent", signature);
    }
}

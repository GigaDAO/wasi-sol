#![allow(deprecated)]

use std::str::FromStr;

use js_sys::Promise;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use emitter_rs::EventEmitter;
use log::info;
use serde::{Deserialize, Serialize};

use anyhow::Result;

use solana_client_wasm::WasmClient as RpcClient;
use solana_sdk::{
    bs58,
    pubkey::Pubkey,
    signature::{Signature, Signer},
    signer::keypair::Keypair,
};

use crate::{
    core::{
        error::WalletError,
        traits::{WalletAdapter, WalletAdapterEvents},
        transaction::TransactionOrVersionedTransaction,
    },
    solana::object::SOLANA,
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum WalletReadyState {
    Installed,
    NotDetected,
    Loadable,
    Unsupported,
}

#[derive(Clone, PartialEq)]
pub struct BaseWalletAdapter {
    name: String,
    url: String,
    icon: String,
    ready_state: WalletReadyState,
    public_key: Option<Pubkey>,
    connecting: bool,
    pub emitter: EventEmitter,
}

impl BaseWalletAdapter {
    pub fn new(name: &str, url: &str, icon: &str) -> Self {
        let ready_state = if cfg!(target_arch = "wasm32") {
            WalletReadyState::Unsupported
        } else {
            WalletReadyState::NotDetected
        };

        BaseWalletAdapter {
            ready_state,
            name: name.to_string(),
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
        self.name.to_string()
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

    async fn auto_connect(&mut self) -> Result<(), WalletError> {
        self.connect().await
    }

    async fn connect(&mut self) -> Result<(), WalletError> {
        info!("Connecting to wallet...");

        if self.connecting {
            self.emit_error(WalletError::WalletConnectionError);
            return Err(WalletError::WalletConnectionError);
        }

        self.connecting = true;

        let options = js_sys::Object::new();
        js_sys::Reflect::set(
            &options,
            &JsValue::from_str("onlyIfTrusted"),
            &JsValue::from_bool(true),
        )
        .unwrap();
        let promise: Promise = SOLANA.sign_in(&options);
        let result = JsFuture::from(promise).await;

        match result {
            Ok(_) => {
                info!("Wallet connected");
                let key = SOLANA.publicKey();
                if key.is_undefined() {
                    info!("Public key is undefined");
                } else {
                    let key_str: String = JsValue::into_serde(&key).unwrap();

                    let public_key = Pubkey::from_str(&key_str).unwrap();
                    info!("Connected to wallet with public key: {:?}", public_key);
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

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), WalletError> {
        info!("Disconnecting from wallet...");

        if self.public_key.is_none() {
            self.emit_error(WalletError::WalletDisconnectedError);
            return Err(WalletError::WalletDisconnectedError);
        }

        self.public_key = None;
        self.ready_state = WalletReadyState::NotDetected;
        self.emit_disconnect();

        let promise: Promise = SOLANA.disconnect();
        let result = JsFuture::from(promise).await;

        match result {
            Ok(_) => {
                info!("Disconnected from wallet");
            }
            Err(err) => {
                log::error!("Failed to disconnect wallet: {:?}", err);
            }
        }

        Ok(())
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

    async fn sign_message(&mut self, keypair: Keypair, message: &str) -> String {
        let message_bytes = message.as_bytes();

        let signature = keypair.sign_message(message_bytes);

        bs58::encode(signature).into_string()
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

#![allow(async_fn_in_trait)]

use anyhow::Result;

use solana_client_wasm::WasmClient as RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature, transaction::Transaction};

use crate::core::{
    error::WalletError, transaction::TransactionOrVersionedTransaction, wallet::WalletReadyState,
};

pub trait WalletAdapterEvents {
    fn emit_connect(&mut self, public_key: Pubkey);
    fn emit_disconnect(&mut self);
    fn error(&mut self, error: WalletError);
    fn ready_state_change(&mut self, ready_state: WalletReadyState);
    fn emit_transaction_sent(&mut self, signature: Signature);
}

pub trait WalletAdapter: WalletAdapterEvents + Send + Sync {
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    fn icon(&self) -> &str;
    fn ready_state(&self) -> WalletReadyState;
    fn public_key(&self) -> Option<Pubkey>;
    fn connecting(&self) -> bool;
    fn connected(&self) -> bool {
        self.public_key().is_some()
    }

    async fn auto_connect(&mut self) -> Result<(), WalletError>;
    async fn connect(&mut self) -> Result<(), WalletError>;
    async fn disconnect(&mut self) -> Result<(), WalletError>;
    async fn send_transaction(
        &mut self,
        client: RpcClient,
        transaction: TransactionOrVersionedTransaction,
    ) -> Result<Signature, WalletError>;
}

pub trait SignerWalletAdapter: WalletAdapter {
    async fn sign_transaction(&self, transaction: &mut Transaction) -> Result<()>;

    async fn sign_all_transactions(&self, transactions: &mut [Transaction]) -> Result<()>;
}

pub trait MessageSignerWalletAdapter: SignerWalletAdapter {
    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>>;
}

pub trait SignInMessageSignerWalletAdapter: MessageSignerWalletAdapter {
    async fn sign_in(&self) -> Result<()>;
}

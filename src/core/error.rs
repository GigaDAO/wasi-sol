use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum WalletError {
    #[error("Wallet not connected")]
    WalletNotConnectedError,
    #[error("Wallet not ready")]
    WalletNotReadyError,
    #[error("Failed to load wallet")]
    WalletLoadError,
    #[error("Wallet configuration error")]
    WalletConfigError,
    #[error("Failed to connect to wallet")]
    WalletConnectionError,
    #[error("Wallet disconnected")]
    WalletDisconnectedError,
    #[error("Failed to disconnect wallet")]
    WalletDisconnectionError,
    #[error("Wallet account error")]
    WalletAccountError,
    #[error("Invalid public key")]
    WalletPublicKeyError,
    #[error("Failed to send transaction")]
    WalletSendTransactionError,
    #[error("Failed to sign transaction")]
    WalletSignTransactionError,
    #[error("Failed to sign message")]
    WalletSignMessageError,
    #[error("Failed to sign in")]
    WalletSignInError,
    #[error("Wallet operation timed out")]
    WalletTimeoutError,
    #[error("Wallet window blocked")]
    WalletWindowBlockedError,
    #[error("Wallet window closed")]
    WalletWindowClosedError,
}

pub type Result<T> = std::result::Result<T, WalletError>;

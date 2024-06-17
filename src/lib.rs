#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod core;
pub mod provider;
pub mod solana;

// re-export
pub use solana_sdk::pubkey;
pub use wasm_bindgen_futures::spawn_local;

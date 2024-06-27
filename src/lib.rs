#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub(crate) mod adapter;
pub mod core;
pub mod provider;

// re-export
pub use solana_sdk::*;
pub use wasm_bindgen_futures::spawn_local;

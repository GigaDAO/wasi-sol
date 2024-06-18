<div align="center">

# 🦀 Wasi Sol

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Netlify Status](https://api.netlify.com/api/v1/badges/d7858d73-f54a-4d4f-878f-466168d8ea07/deploy-status)](https://app.netlify.com/sites/wasi-sol/deploys)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![Crates.io](https://img.shields.io/crates/v/wasi-sol.svg)](https://crates.io/crates/wasi-sol)
[![Crates.io Downloads](https://img.shields.io/crates/d/wasi-sol)](https://crates.io/crates/wasi-sol)
[![docs](https://docs.rs/wasi-sol/badge.svg)](https://docs.rs/wasi-sol/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![GigaDAO Discord](https://dcbadge.limes.pink/api/server/gigadao-gigadex-now-live-920188005686337566)](https://discord.gg/gigadao-gigadex-now-live-920188005686337566)

![WASI SOL Demo](https://github.com/wiseaidev/wasi-sol/assets/62179149/ed3668ef-6f18-4d74-a10d-5e1ae551c695)

</div>

A Solana Wallet adapter for WASM frameworks.

## 🔒 Wallets Support

> [!NOTE]
> By default, this crate triggers all `EIP-1193` compatible wallets, but you can only connect and perform all actions listed below if it is Phantom wallet.

| Wallet    | Supported   | Features          |
|-----------|-------------|-------------------|
| Phantom   | ✅          | All               |
| Metamask  | ❌          | Wallet Connect Only |
| Solflare  | ❌          | ❌                |

## 🌐 Wasm Frameworks Support

| Framework | Supported   |
|-----------|-------------|
| Yew       | ✅          |
| Dioxus    | ❌          |
| Leptos    | ❌          |

## ⚙️ Features

| Method                | Supported | Tested |
|-----------------------|-----------|--------|
| `connect`             | ✅        | ✅     |
| `disconnect`          | ✅        | ✅     |
| `send_transaction`    | ✅        | ✅     |
| `sign_message`        | ❌        | ❌     |
| `sign_transaction`    | ❌        | ❌     |
| `sign_all_transactions` | ❌      | ❌     |
| `sign_in`             | ❌        | ❌     |

❌: TODO

## 🚀 Examples

In addition to the [`examples`](examples) directory, you can use the following snippet of code to add `wasi-sol` wallet adapter using its built-in providers and hooks:

```rust , ignore
use yew::prelude::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::BaseWalletAdapter,
    provider::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    spawn_local
};

#[function_component]
pub fn App() -> Html {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![BaseWalletAdapter::new(
        "Phantom",
        "https://phantom.app",
        "phantom_icon_url",
    )];

    html! {
        <ConnectionProvider {endpoint}>
            <WalletProvider {endpoint} {wallets}>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[function_component]
pub fn LoginPage() -> Html {
    let _connection_context = use_connection();
    let wallet_context = use_wallet();
    let connected = use_state(|| false);
    let wallet_adapter = use_state(|| wallet_context);

    let wallet_info = (*wallet_adapter).clone();

    let connect_wallet = {
        let connected = connected.clone();
        let wallet_adapter = wallet_adapter.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let wallet_adapter = wallet_adapter.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

                match wallet_info.connect().await {
                    Ok(_) => {
                        wallet_adapter.set(wallet_info);
                        connected.set(true);
                    }
                    Err(err) => {
                        log::error!("Failed to connect wallet: {}", err);
                    }
                }
            });
        })
    };

    let disconnect_wallet = {
        let connected = connected.clone();
        let wallet_adapter = wallet_adapter.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let wallet_adapter = wallet_adapter.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

                match wallet_info.disconnect().await {
                    Ok(_) => {
                        wallet_adapter.set(wallet_info);
                        connected.set(false);
                    }
                    Err(err) => {
                        log::error!("Failed to disconnect wallet: {}", err);
                    }
                }
            });
        })
    };

    html! {
        <div class="content">
            <div class="wallet-info">
                if *connected {
                    if let Some(ref key) = wallet_info.public_key() {
                        <p>{ format!("Connected Wallet: {}", wallet_info.name()) }</p>
                        <p>{ format!("Connected Public Key: {}", key) }</p>
                    } else {
                        <p>{ "Connected but no wallet info available" }</p>
                    }
                }
            </div>
            <div class="buttons">
                if !*connected {
                    <button class="connect-button" onclick={connect_wallet}>
                        <img src="images/phantom_logo.png" alt="Phantom Wallet" class="button-icon" />
                        { "Connect Wallet" }
                    </button>
                } else {
                    <button class="disconnect-button" onclick={disconnect_wallet}>
                        <img src="images/phantom_logo.png" alt="Disconnect Wallet" class="button-icon" />
                        { "Disconnect Wallet" }
                    </button>
                }
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
```
## 🎧 Event Listener

This crate implements a handy event listener pattern with a built-in `emitter` object that you can use to subscribe to particular events. This functionality allows you to set state in the UI, perform actions on wallet connect, and more.


```rust , ignore
// ...snip...

#[function_component]
pub fn LoginPage() -> Html {
    let wallet_context = use_wallet();
    let connected = use_state(|| false);
    let wallet_adapter = use_state(|| wallet_context);

    let connect_wallet = {
        // ...snip...

        Callback::from(move |_| {
            // ...snip...

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

                wallet_info.emitter.on("connect", move |public_key: Pubkey| {
                    log::info!("Event Listener: Got pubkey {}", public_key);
                    wallet_adapter.set(wallet_info);
                    connected.set(true);
                });

                match wallet_info.connect().await {
                    Ok(_) => {
                    }
                    Err(err) => {
                        log::error!("Failed to connect wallet: {}", err);
                    }
                }
            });
        })
    };

    // ...snip...

    html! {
        <>
        </>
    }
}
```

## 👥 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/gigadao/wasi-sol). Your contributions help improve this library for the community.

## 📝 License

This project is licensed under the [MIT License](LICENSE).

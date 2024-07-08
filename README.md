<div align="center">

# 🦀 Wasi Sol

![wasi-sol-logo](https://github.com/GigaDAO/wasi-sol/assets/62179149/faac3b2c-4c6e-41e9-87f9-34506f3b21bd)

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Netlify Status](https://api.netlify.com/api/v1/badges/d7858d73-f54a-4d4f-878f-466168d8ea07/deploy-status)](https://wasi-sol.netlify.app/)
[![Netlify Status](https://api.netlify.com/api/v1/badges/15f88b9f-edfd-4e94-9bca-2150b95343ca/deploy-status)](https://wasi-sol-dio.netlify.app)
[![Netlify Status](https://api.netlify.com/api/v1/badges/21898514-21da-4a2d-a50f-1e8fad55dd2a/deploy-status)](https://wasi-sol-lep.netlify.app/)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![Crates.io](https://img.shields.io/crates/v/wasi-sol.svg)](https://crates.io/crates/wasi-sol)
[![Crates.io Downloads](https://img.shields.io/crates/d/wasi-sol)](https://crates.io/crates/wasi-sol)
[![docs](https://docs.rs/wasi-sol/badge.svg)](https://docs.rs/wasi-sol/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&amp%3Bref=main&amp%3Brepo=816037127&skip_quickstart=true&repo=816037127&machine=basicLinux32gb&ref=main&geo=EuropeWest)

[![GigaDAO Discord](https://dcbadge.limes.pink/api/server/gigadao-gigadex-now-live-920188005686337566)](https://discord.gg/gigadao-gigadex-now-live-920188005686337566)

| Framework | Demo |
| --- | --- |
| Yew | ![yew-demo](https://github.com/GigaDAO/wasi-sol/assets/62179149/a77daf37-487a-446c-8acd-9d46427503ad) |
| Dioxus | ![dioxus-demo](https://github.com/GigaDAO/wasi-sol/assets/62179149/8f2f4dda-aae2-4bb4-ad1d-1794b10d8949) |
| Leptos | ![leptos-demo](https://github.com/GigaDAO/wasi-sol/assets/62179149/ee09d80b-2355-45a3-b1b5-2a0876b780fe) |

</div>

A Solana Wallet adapter for WASM frameworks.

## 🔒 Wallets Support

| Wallet    | Supported   | Features          |
|-----------|-------------|-------------------|
| Phantom   | ✅          | All               |
| Solflare  | ✅          | All               |
| Backpack  | ✅          | All               |

## 🌐 Wasm Frameworks Support

| Framework | Supported   |
|-----------|-------------|
| Yew       | ✅          |
| Dioxus    | ✅          |
| Leptos    | ✅          |

## ⚙️ Features

| Method                | Supported | Tested |
|-----------------------|-----------|--------|
| `connect`             | ✅        | ✅     |
| `disconnect`          | ✅        | ✅     |
| `sign_in`             | ✅        | ✅     |
| `sign_message`        | ✅        | ✅     |
| `sign_transaction`    | ✅        | ✅     |
| `send_transaction`    | ✅        | ✅     |

## 🔥 Getting Started

Wasi Sol provides providers and hooks that you can use to bring all wallet adapter functionalities to your app. To begin, wrap your main `App` component with the corresponding providers:

```rust , ignore
// Yew Component

#[function_component]
pub fn App() -> Html {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        Wallet::Phantom.into(),
        Wallet::Solflare.into(),
        Wallet::Backpack.into(),
    ];

    html! {
        <ConnectionProvider {endpoint}>
            <WalletProvider {wallets}>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}
```

This will allow you to use the hooks to create the wallet adapter that exists in the wallets vector:

```rust , ignore
// Yew Component

#[function_component]
pub fn LoginPage() -> Html {
    let phantom_context = use_wallet::<Wallet>(Wallet::Phantom);
    let solflare_context = use_wallet::<Wallet>(Wallet::Solflare);
    let backpack_context = use_wallet::<Wallet>(Wallet::Backpack);

    // ...snip...

    html! {
        <>
        </>
    }
}
```

Now you can choose the wallets you want to add to allow users to connect to. Wasi Sol comes with built-in reusable components that encapsulate all connect and disconnect logic so that you can develop web apps quickly:

```rust , ignore
// Yew Component

#[function_component]
pub fn LoginPage() -> Html {
    // ...snip...

    html! {
        <LoginForm
            phantom={Some(phantom_wallet_adapter)}
            solflare={Some(solflare_wallet_adapter)}
            backpack={None}
            {connected}
        />
    }
}
```

This will select the Phantom and Solflare wallets and allow users to connect them to the app. The Backpack wallet is disabled in this case.

More detailed implementations can be found in the examples below.

## 🚀 Examples

| Framework | Example   |
|-----------|-------------|
| Yew       | [![Github](https://img.shields.io/badge/Open-Github-181717.svg?logo=github&logoColor=white)](./examples/yew)         |
| Dioxus    | [![Github](https://img.shields.io/badge/Open-Github-181717.svg?logo=github&logoColor=white)](./examples/dioxus)          |
| Leptos    | [![Github](https://img.shields.io/badge/Open-Github-181717.svg?logo=github&logoColor=white)](./examples/leptos)             |

## 🎧 Event Listener

![Event Emitter Pattern](https://github.com/GigaDAO/wasi-sol/assets/62179149/65edfdc2-d86c-464a-a67f-5ef08099adc6)

This crate implements a handy event listener pattern with a built-in `emitter` object that you can use to subscribe to particular events. This functionality allows you to set state in the UI, perform actions on wallet connect, and more.


```rust , ignore
// Yew Component
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

![event emitter demo](https://github.com/GigaDAO/wasi-sol/assets/62179149/8d271384-9565-47dc-8e26-212ddf3bdfc0)

## 👥 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/gigadao/wasi-sol). Your contributions help improve this library for the community.

## 📝 License

This project is licensed under the [MIT License](LICENSE).

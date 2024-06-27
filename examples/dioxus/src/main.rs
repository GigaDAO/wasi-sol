use dioxus::prelude::*;
use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::{BaseWalletAdapter, Wallet},
    provider::dioxus::{
        connection::ConnectionProvider,
        wallet::{use_wallet, WalletProvider},
    },
};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    launch(app);
}

fn app() -> Element {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        BaseWalletAdapter::new("Solflare", "https://solflare.com", "./solflare_logo.png"),
        BaseWalletAdapter::new("Phantom", "https://phantom.app", "./phantom_logo.png"),
    ];

    rsx! {
        ConnectionProvider {
            endpoint: endpoint,
            WalletProvider {
                wallets: wallets,
                LoginPage {}
            }
        }
    }
}

#[component]
fn LoginPage() -> Element {
    let phantom_context = use_wallet(Wallet::Phantom);
    let solflare_context = use_wallet(Wallet::Solflare);
    let phantom_wallet_adapter = use_signal(|| phantom_context);
    let solflare_wallet_adapter = use_signal(|| solflare_context);
    let mut connected = use_signal(|| false);
    let phantom_wallet_info = (*phantom_wallet_adapter)().clone();
    let solflare_wallet_info = (*solflare_wallet_adapter)().clone();
    let error = use_signal(|| None as Option<String>);

    let connect_wallet_phantom = move |_| {
        let mut phantom_wallet_adapter = phantom_wallet_adapter.clone();

        spawn(async move {
            let mut phantom_wallet_info = (*phantom_wallet_adapter)().clone();

            match phantom_wallet_info.connect().await {
                Ok(_) => {
                    phantom_wallet_adapter.set(phantom_wallet_info);
                    connected.set(true);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                }
            }
        });
    };

    let connect_wallet_solflare = move |_| {
        let mut solflare_wallet_adapter = solflare_wallet_adapter.clone();

        spawn(async move {
            let mut solflare_wallet_info = (*solflare_wallet_adapter)().clone();

            match solflare_wallet_info.connect().await {
                Ok(_) => {
                    solflare_wallet_adapter.set(solflare_wallet_info);
                    connected.set(true);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                }
            }
        });
    };

    let disconnect_wallet = move |_| {
        let mut phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let mut solflare_wallet_adapter = solflare_wallet_adapter.clone();

        spawn(async move {
            let mut phantom_wallet_info = (*phantom_wallet_adapter)().clone();
            let mut solflare_wallet_info = (*solflare_wallet_adapter)().clone();

            match phantom_wallet_info.disconnect().await {
                Ok(_) => {
                    phantom_wallet_adapter.set(phantom_wallet_info);
                    connected.set(false);
                }
                Err(_err) => {}
            }

            match solflare_wallet_info.disconnect().await {
                Ok(_) => {
                    solflare_wallet_adapter.set(solflare_wallet_info);
                    connected.set(false);
                }
                Err(_err) => {}
            }
        });
    };

    rsx! {
        div {
            class: "wallet-adapter",
            header {
                class: "header",
                img {
                    src: "./header.svg",
                    alt: "Phantom Wallet",
                    class: "button-icon"
                },
                h1 { "Wasi Sol Dioxus Wallet Adapter" }
            },
            div {
                class: "content",
                div {
                    class: "wallet-info",
                    if (*connected)() {
                        if let Some(ref key) = phantom_wallet_info.public_key() {
                            p { "Connected Wallet: {phantom_wallet_info.name()}" }
                            p { "Connected Public Key: {key}" }
                        } else if let Some(ref key) = solflare_wallet_info.public_key() {
                            p { "Connected Wallet: {solflare_wallet_info.name()}" }
                            p { "Connected Public Key: {key}" }
                        } else {
                            p { "Connected but no wallet info available" }
                        }
                    }
                },
                div {
                    class: "buttons",
                    if !(*connected)() {
                        button {
                            class: "connect-button-phantom",
                            onclick: connect_wallet_phantom,
                            img {
                                src: phantom_wallet_info.icon(),
                                alt: "Phantom Wallet",
                                class: "button-icon-phantom"
                            },
                            "Connect Wallet"
                        }
                        button {
                            class: "connect-button-solflare",
                            onclick: connect_wallet_solflare,
                            img {
                                src: solflare_wallet_info.icon(),
                                alt: "Solflare Wallet",
                                class: "button-icon-solflare"
                            },
                            "Connect Wallet"
                        }
                    } else if let Some(ref _key) = phantom_wallet_info.public_key() {
                        button {
                            class: "disconnect-button",
                            onclick: disconnect_wallet,
                            img {
                                src: phantom_wallet_info.icon(),
                                alt: "Disconnect Wallet",
                                class: "button-icon"
                            },
                            "Disconnect Wallet"
                        }
                        } else if let Some(ref _key) = solflare_wallet_info.public_key() {
                        button {
                            class: "disconnect-button",
                            onclick: disconnect_wallet,
                            img {
                                src: solflare_wallet_info.icon(),
                                alt: "Disconnect Wallet",
                                class: "button-icon"
                            },
                            "Disconnect Wallet"
                        }
                        },
                    if let Some(ref e) = (*error)() {
                        p {
                            style: "color: red;",
                            { e.clone() }
                        }
                    }
                },
            },
            footer {
                class: "footer",
                p { "2024 GigaDAO Foundation." }
            }
        }
    }
}

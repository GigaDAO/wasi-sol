use dioxus::prelude::*;
use wasi_sol::core::traits::WalletAdapter;
use wasi_sol::core::wallet::BaseWalletAdapter;
use wasi_sol::provider::dioxus::connection::ConnectionProvider;
use wasi_sol::provider::dioxus::wallet::use_wallet;
use wasi_sol::provider::dioxus::wallet::WalletProvider;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    launch(app);
}

fn app() -> Element {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![BaseWalletAdapter::new(
        "Phantom",
        "https://phantom.app",
        "phantom_icon_url",
    )];

    rsx! {
        ConnectionProvider {
            endpoint: endpoint,
            WalletProvider {
                wallets: wallets,
                endpoint: endpoint,
                LoginPage {}
            }
        }
    }
}

#[component]
fn LoginPage() -> Element {
    let wallet_context = use_wallet();
    let wallet_adapter = use_signal(|| wallet_context);
    let mut connected = use_signal(|| false);
    let wallet_info = (*wallet_adapter)().clone();
    let mut error = use_signal(|| None as Option<String>);

    let connect_wallet = move |_| {
        let mut wallet_adapter = wallet_adapter.clone();

        spawn(async move {
            let mut wallet_info = (*wallet_adapter)().clone();

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
    };

    let disconnect_wallet = move |_| {
        let mut wallet_adapter = wallet_adapter.clone();

        spawn(async move {
            let mut wallet_info = (*wallet_adapter)().clone();

            match wallet_info.disconnect().await {
                Ok(_) => {
                    wallet_adapter.set(wallet_info);
                    connected.set(false);
                }
                Err(err) => {
                    log::error!("Failed to disconnect wallet: {}", err);
                    error.set(Some(err.to_string()));
                }
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
                        if let Some(ref key) = wallet_info.public_key() {
                            p { "Connected Wallet: {wallet_info.name()}" }
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
                            class: "connect-button",
                            onclick: connect_wallet,
                            img {
                                src: "./phantom_logo.png",
                                alt: "Phantom Wallet",
                                class: "button-icon"
                            },
                            "Connect Wallet"
                        }
                    } else {
                        button {
                            class: "disconnect-button",
                            onclick: disconnect_wallet,
                            img {
                                src: "./phantom_logo.png",
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

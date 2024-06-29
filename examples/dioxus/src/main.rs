use dioxus::prelude::*;
use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::{BaseWalletAdapter, Wallet},
    provider::dioxus::{
        connection::ConnectionProvider,
        wallet::{use_wallet, WalletProvider},
    },
    forms::dioxus::login::LoginForm
};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    launch(app);
}

fn app() -> Element {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        BaseWalletAdapter::new(Wallet::Solflare, "https://solflare.com", "./solflare_logo.png"),
        BaseWalletAdapter::new(Wallet::Phantom, "https://phantom.app", "./phantom_logo.png"),
        BaseWalletAdapter::new(Wallet::Backpack, "https://backpack.app", "./backpack_logo.png"),
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
    let backpack_context = use_wallet(Wallet::Backpack);
    let phantom_wallet_adapter = use_signal(|| phantom_context);
    let solflare_wallet_adapter = use_signal(|| solflare_context);
    let backpack_wallet_adapter = use_signal(|| backpack_context);
    let connected = use_signal(|| false);
    let phantom_wallet_info = (*phantom_wallet_adapter)().clone();
    let solflare_wallet_info = (*solflare_wallet_adapter)().clone();

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
            },
            LoginForm {
                phantom: phantom_wallet_adapter,
                solflare: solflare_wallet_adapter,
                backpack: backpack_wallet_adapter,
                connected: connected
            }
            footer {
                class: "footer",
                p { "2024 GigaDAO Foundation." }
            }
        }
    }
}

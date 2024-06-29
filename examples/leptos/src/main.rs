use leptos::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::{BaseWalletAdapter, Wallet},
    forms::leptos::login::LoginForm,
    provider::leptos::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
};

#[component]
pub fn App() -> impl IntoView {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        BaseWalletAdapter::new(
            Wallet::Solflare,
            "https://solflare.com",
            "images/solflare_logo.png",
        ),
        BaseWalletAdapter::new(
            Wallet::Phantom,
            "https://phantom.app",
            "images/phantom_logo.png",
        ),
        BaseWalletAdapter::new(
            Wallet::Backpack,
            "https://backpack.app",
            "images/backpack_logo.png",
        ),
    ];

    view! {
        <ConnectionProvider endpoint=endpoint>
            <WalletProvider wallets=wallets>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let _connection_context = use_connection();
    let phantom_context = use_wallet(Wallet::Phantom);
    let solflare_context = use_wallet(Wallet::Solflare);
    let backpack_context = use_wallet(Wallet::Backpack);
    let (connected, set_connected) = create_signal(false);
    let (phantom_wallet_adapter, set_phantom_wallet_adapter) = create_signal(phantom_context);
    let (solflare_wallet_adapter, set_solflare_wallet_adapter) = create_signal(solflare_context);
    let (backpack_wallet_adapter, set_sbackpack_wallet_adapter) = create_signal(backpack_context);

    view! {
        <div class="wallet-adapter">
            <header class="header">
                <img src="images/leptos-logo.png" alt="Leptos Logo" class="leptos-logo" />
                <h1>"Wasi Sol Wallet Adapter"</h1>
            </header>
            <div class="content">
                <div class="wallet-info">
                    {move ||
                        if connected.get() {
                            Some(view!{
                                {move ||
                                    if let Some(key) = phantom_wallet_adapter.get().public_key() {
                                        view!{
                                            <p>"Connected Wallet: " {move || phantom_wallet_adapter.get().name()} </p>
                                            <p>"Connected Public Key: " {move || key.to_string() } </p>
                                        }
                                    } else if let Some(key) = solflare_wallet_adapter.get().public_key() {
                                        view!{
                                            <p>"Connected Wallet: " {move || solflare_wallet_adapter.get().name()} </p>
                                            <p>"Connected Public Key: " {move || key.to_string() } </p>
                                        }
                                    } else {
                                        view!{
                                            <p>"Connected but no wallet info available"</p>
                                            <p>{}</p>
                                        }
                                    }
                                }
                            })
                        } else {
                            None
                        }
                    }
                </div>
                <LoginForm
                    phantom=(phantom_wallet_adapter, set_phantom_wallet_adapter)
                    solflare=(solflare_wallet_adapter, set_solflare_wallet_adapter)
                    backpack=(backpack_wallet_adapter, set_sbackpack_wallet_adapter)
                    connected=(connected, set_connected)
                 />
            </div>
            <footer class="footer">
                <p>"2024 GigaDAO Foundation."</p>
            </footer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| view! { <App/> })
}

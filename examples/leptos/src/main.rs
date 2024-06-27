use leptos::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::{BaseWalletAdapter, Wallet},
    provider::leptos::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    pubkey::Pubkey,
    spawn_local,
};

#[component]
pub fn App() -> impl IntoView {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        BaseWalletAdapter::new(
            "Solflare",
            "https://solflare.com",
            "images/solflare_logo.png",
        ),
        BaseWalletAdapter::new("Phantom", "https://phantom.app", "images/phantom_logo.png"),
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
    let (connected, set_connected) = create_signal(false);
    let (phantom_wallet_adapter, set_phantom_wallet_adapter) = create_signal(phantom_context);
    let (solflare_wallet_adapter, set_solflare_wallet_adapter) = create_signal(solflare_context);
    let (error, set_error) = create_signal(String::default());

    let connect_phantom_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = phantom_wallet_adapter.get_untracked();

            wallet_info
                .emitter
                .on("connect", move |public_key: Pubkey| {
                    log::info!("Event Listener: Got pubkey {}", public_key);
                });

            match wallet_info.connect().await {
                Ok(_) => {
                    set_phantom_wallet_adapter.set(wallet_info);
                    set_connected.set(true);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                    set_error.set(err.to_string());
                }
            }
        });
    };

    let connect_solflare_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = solflare_wallet_adapter.get_untracked();

            wallet_info
                .emitter
                .on("connect", move |public_key: Pubkey| {
                    log::info!("Event Listener: Got pubkey {}", public_key);
                });

            match wallet_info.connect().await {
                Ok(_) => {
                    set_phantom_wallet_adapter.set(wallet_info);
                    set_connected.set(true);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                    set_error.set(err.to_string());
                }
            }
        });
    };

    let disconnect_wallet = move |_| {
        spawn_local(async move {
            let mut phantom_wallet_info = phantom_wallet_adapter.get_untracked();
            let mut solflare_wallet_info = solflare_wallet_adapter.get_untracked();

            match phantom_wallet_info.disconnect().await {
                Ok(_) => {
                    set_phantom_wallet_adapter.set(phantom_wallet_info);
                    set_connected.set(false);
                }
                Err(_err) => {}
            }
            match solflare_wallet_info.disconnect().await {
                Ok(_) => {
                    set_solflare_wallet_adapter.set(solflare_wallet_info);
                    set_connected.set(false);
                }
                Err(_err) => {}
            }
        });
    };

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
                <div class="buttons">
                    {move ||
                        if !connected.get() {
                            view!{
                                <button class="connect-button-phantom" on:click=connect_phantom_wallet>
                                    <img src={phantom_wallet_adapter.get().icon()} alt="Phantom Wallet" class="button-icon" />
                                    "Connect Wallet"
                                </button>
                                <button class="connect-button-solflare" on:click=connect_solflare_wallet>
                                    <img src={solflare_wallet_adapter.get().icon()} alt="Phantom Wallet" class="button-icon" />
                                    "Connect Wallet"
                                </button>
                            }
                        } else if let Some(_key) = phantom_wallet_adapter.get().public_key() {
                                        view!{
                                <button class="disconnect-button" on:click=disconnect_wallet>
                                    <img src={phantom_wallet_adapter.get().icon()} alt="Phantom Wallet" class="button-icon" />
                                    "Disconnect Wallet"
                                </button>
                                <>
                                </>
                                        }
                                    } else if let Some(_key) = solflare_wallet_adapter.get().public_key() {
                                        view!{
                                <button class="disconnect-button" on:click=disconnect_wallet>
                                    <img src={solflare_wallet_adapter.get().icon()} alt="Phantom Wallet" class="button-icon" />
                                    "Disconnect Wallet"
                                </button>
                                <>
                                </>
                                        }
                                    } else {
                                        view!{
                                <button>
                                </button>
                                <>
                                </>
                                        }
                                    }
                    }
                    {move ||
                        if !error.get().is_empty() {
                            view!{
                                <p style="color: red;">{ error.get() }</p>
                            }
                        }
                        else {
                            view!{
                                <p></p>
                            }
                        }
                    }
                </div>
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

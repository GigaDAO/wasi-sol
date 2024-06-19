use yew::prelude::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::BaseWalletAdapter,
    provider::yew::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    pubkey::Pubkey,
    spawn_local,
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
    let error = use_state(|| None as Option<String>);

    let connect_wallet = {
        let connected = connected.clone();
        let wallet_adapter = wallet_adapter.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let wallet_adapter = wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

                wallet_info
                    .emitter
                    .on("connect", move |public_key: Pubkey| {
                        log::info!("Event Listener: Got pubkey {}", public_key);
                    });

                match wallet_info.connect().await {
                    Ok(_) => {
                        wallet_adapter.set(wallet_info);
                        connected.set(true);
                    }
                    Err(err) => {
                        log::error!("Failed to connect wallet: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let disconnect_wallet = {
        let connected = connected.clone();
        let wallet_adapter = wallet_adapter.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let wallet_adapter = wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

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
        })
    };

    html! {
        <div class="wallet-adapter">
            <header class="header">
                <img src="images/logo.jpeg" alt="Yew Logo" class="yew-logo" />
                <h1>{ "Wasi Sol Wallet Adapter" }</h1>
            </header>
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
                    if let Some(ref e) = *error {
                        <p style="color: red;">{ e.clone() }</p>
                    }
                </div>
            </div>
            <footer class="footer">
                <p>{ "2024 GigaDAO Foundation." }</p>
            </footer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

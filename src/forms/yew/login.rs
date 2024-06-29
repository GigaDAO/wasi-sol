use web_sys::window;
use yew::prelude::*;

use crate::adapter::{backpack::XNFT, phantom::SOLANA, solflare::SOLFLARE};
use crate::core::traits::WalletAdapter;
use crate::core::wallet::BaseWalletAdapter;
use crate::pubkey::Pubkey;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub phantom: UseStateHandle<BaseWalletAdapter>,
    pub solflare: UseStateHandle<BaseWalletAdapter>,
    pub backpack: UseStateHandle<BaseWalletAdapter>,
    pub connected: UseStateHandle<bool>,
}

#[function_component]
pub fn LoginForm(props: &Props) -> Html {
    let connected = &props.connected;

    let phantom_wallet_adapter = &props.phantom;
    let solflare_wallet_adapter = &props.solflare;
    let backpack_wallet_adapter = &props.backpack;

    let phantom_wallet_info = (*phantom_wallet_adapter).clone();
    let solflare_wallet_info = (*solflare_wallet_adapter).clone();
    let backpack_wallet_info = (*backpack_wallet_adapter).clone();

    let error = use_state(|| None as Option<String>);

    let connect_phantom_wallet = {
        let connected = connected.clone();
        let phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let error = error.clone();
        let url = phantom_wallet_info.clone().url();

        Callback::from(move |_| {
            let connected = connected.clone();
            let phantom_wallet_adapter = phantom_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut phantom_wallet_info = (*phantom_wallet_adapter).clone();

                phantom_wallet_info
                    .emitter
                    .on("connect", move |public_key: Pubkey| {
                        log::info!("Event Listener: Got pubkey {}", public_key);
                    });

                match phantom_wallet_info.connect().await {
                    Ok(_) => {
                        phantom_wallet_adapter.set(phantom_wallet_info);
                        connected.set(true);
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                        let window = window().expect("no global `window` exists");
                        window
                            .open_with_url(&phantom_wallet_info.url())
                            .expect("failed to open a new tab");
                    }
                }
            });

            if SOLANA.is_undefined() {
                let window = window().expect("no global `window` exists");
                window
                    .open_with_url(&url)
                    .expect("failed to open a new tab");
            }
        })
    };

    let connect_solflare_wallet = {
        let connected = connected.clone();
        let solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let error = error.clone();
        let url = solflare_wallet_info.clone().url();

        Callback::from(move |_| {
            let connected = connected.clone();
            let solflare_wallet_adapter = solflare_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut solflare_wallet_info = (*solflare_wallet_adapter).clone();

                solflare_wallet_info
                    .emitter
                    .on("connect", move |public_key: Pubkey| {
                        log::info!("Event Listener: Got pubkey {}", public_key);
                    });

                match solflare_wallet_info.connect().await {
                    Ok(_) => {
                        solflare_wallet_adapter.set(solflare_wallet_info);
                        connected.set(true);
                    }
                    Err(err) => {
                        log::info!("Event Listener: Got pubkey {}", err);
                        error.set(Some(err.to_string()));
                        let window = window().expect("no global `window` exists");
                        window
                            .open_with_url(&solflare_wallet_info.url())
                            .expect("failed to open a new tab");
                    }
                }
            });

            if SOLFLARE.is_undefined() {
                let window = window().expect("no global `window` exists");
                window
                    .open_with_url(&url)
                    .expect("failed to open a new tab");
            }
        })
    };

    let connect_backpack_wallet = {
        let connected = connected.clone();
        let backpack_wallet_adapter = backpack_wallet_adapter.clone();
        let error = error.clone();
        let url = backpack_wallet_info.clone().url();

        Callback::from(move |_| {
            let connected = connected.clone();
            let backpack_wallet_adapter = backpack_wallet_adapter.clone();
            let error = error.clone();
            let connected = connected.clone();

            spawn_local(async move {
                let mut backpack_wallet_info = (*backpack_wallet_adapter).clone();

                backpack_wallet_info
                    .emitter
                    .on("connect", move |public_key: Pubkey| {
                        log::info!("Event Listener: Got pubkey {}", public_key);
                    });

                match backpack_wallet_info.connect().await {
                    Ok(_) => {
                        backpack_wallet_adapter.set(backpack_wallet_info.clone());
                        connected.set(true);
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                        let window = window().expect("no global `window` exists");
                        window
                            .open_with_url(&backpack_wallet_info.url())
                            .expect("failed to open a new tab");
                    }
                }
            });

            if XNFT.is_undefined() {
                let window = window().expect("no global `window` exists");
                window
                    .open_with_url(&url)
                    .expect("failed to open a new tab");
            }
        })
    };

    let disconnect_wallet = {
        let connected = connected.clone();
        let solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let backpack_wallet_adapter = backpack_wallet_adapter.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let phantom_wallet_adapter = phantom_wallet_adapter.clone();
            let solflare_wallet_adapter = solflare_wallet_adapter.clone();
            let backpack_wallet_adapter = backpack_wallet_adapter.clone();

            spawn_local(async move {
                let mut phantom_wallet_info = (*phantom_wallet_adapter).clone();
                let mut solflare_wallet_info = (*solflare_wallet_adapter).clone();
                let mut backpack_wallet_info = (*backpack_wallet_adapter).clone();

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
                match backpack_wallet_info.disconnect().await {
                    Ok(_) => {
                        backpack_wallet_adapter.set(backpack_wallet_info);
                        connected.set(false);
                    }
                    Err(_err) => {}
                }
            });
        })
    };

    html! {
        <div class="container">
            <div class="buttons">
                if !**connected {
                    <button
                        onclick={connect_phantom_wallet.clone()}
                    >
                        <img
                            src={phantom_wallet_info.icon()}
                            alt="Phantom Wallet"
                        />
                        { "Connect Phantom Wallet" }
                    </button>
                    <button
                        onclick={connect_solflare_wallet.clone()}
                    >
                        <img
                            src={solflare_wallet_info.icon()}
                            alt="Solflare Wallet"
                        />
                        { "Connect Solflare Wallet" }
                    </button>
                    <button
                        onclick={connect_backpack_wallet.clone()}
                    >
                        <img
                            src={backpack_wallet_info.icon()}
                            alt="Backpack Wallet"
                        />
                        { "Connect Backpack Wallet" }
                    </button>
                } else if let Some(ref _key) = phantom_wallet_info.public_key() {
                    <button class="disconnect" onclick={disconnect_wallet.clone()}>
                        <img
                            src={phantom_wallet_info.icon()}
                            alt="Disconnect Wallet"
                        />
                        { "Disconnect Wallet" }
                    </button>
                } else if let Some(ref _key) = backpack_wallet_info.public_key() {
                    <button class="disconnect" onclick={disconnect_wallet.clone()}>
                        <img
                            src={backpack_wallet_info.icon()}
                            alt="Disconnect Wallet"
                        />
                        { "Disconnect Wallet" }
                    </button>
                } else {
                    <button class="disconnect" onclick={disconnect_wallet.clone()}>
                        <img
                            src={solflare_wallet_info.icon()}
                            alt="Disconnect Wallet"
                        />
                        { "Disconnect Wallet" }
                    </button>
                }
            </div>
            if let Some(ref e) = *error {
                <p class="error-message">{ e.clone() }</p>
            }
        </div>
    }
}

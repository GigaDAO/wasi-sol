use yew::prelude::*;

use crate::core::traits::WalletAdapter;
use crate::core::wallet::BaseWalletAdapter;
use crate::pubkey::Pubkey;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub phantom: Option<UseStateHandle<BaseWalletAdapter>>,
    pub solflare: Option<UseStateHandle<BaseWalletAdapter>>,
    pub backpack: Option<UseStateHandle<BaseWalletAdapter>>,
    pub connected: UseStateHandle<bool>,
}

#[function_component]
pub fn LoginForm(props: &Props) -> Html {
    let connected = &props.connected;

    let phantom_wallet = &props.phantom;
    let solflare_wallet = &props.solflare;
    let backpack_wallet = &props.backpack;

    let mut phantom_wallet_adapter = use_state(|| BaseWalletAdapter::default());
    let mut solflare_wallet_adapter = use_state(|| BaseWalletAdapter::default());
    let mut backpack_wallet_adapter = use_state(|| BaseWalletAdapter::default());

    if phantom_wallet.is_some() {
        phantom_wallet_adapter = phantom_wallet.clone().unwrap();
    }
    if solflare_wallet.is_some() {
        solflare_wallet_adapter = solflare_wallet.clone().unwrap();
    }
    if backpack_wallet.is_some() {
        backpack_wallet_adapter = backpack_wallet.clone().unwrap();
    }

    let phantom_wallet_info = (*phantom_wallet_adapter).clone();
    let solflare_wallet_info = (*solflare_wallet_adapter).clone();
    let backpack_wallet_info = (*backpack_wallet_adapter).clone();

    let error = use_state(|| None as Option<String>);

    let connect_phantom_wallet = {
        let connected = connected.clone();
        let phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let error = error.clone();

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
                    Ok(conn) => {
                        phantom_wallet_adapter.set(phantom_wallet_info);
                        connected.set(conn);
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let connect_solflare_wallet = {
        let connected = connected.clone();
        let solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let error = error.clone();

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
                    Ok(conn) => {
                        solflare_wallet_adapter.set(solflare_wallet_info);
                        connected.set(conn);
                    }
                    Err(err) => {
                        log::info!("Event Listener: Got pubkey {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let connect_backpack_wallet = {
        let connected = connected.clone();
        let backpack_wallet_adapter = backpack_wallet_adapter.clone();
        let error = error.clone();

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
                    Ok(conn) => {
                        backpack_wallet_adapter.set(backpack_wallet_info.clone());
                        connected.set(conn);
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                    }
                }
            });
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
                    if phantom_wallet.is_some() {
                        <button
                            onclick={connect_phantom_wallet.clone()}
                        >
                            <img
                                src={phantom_wallet_info.icon()}
                                alt="Phantom Wallet"
                            />
                            { "Connect Phantom Wallet" }
                        </button>
                    }
                    if solflare_wallet.is_some() {
                        <button
                            onclick={connect_solflare_wallet.clone()}
                        >
                            <img
                                src={solflare_wallet_info.icon()}
                                alt="Solflare Wallet"
                            />
                            { "Connect Solflare Wallet" }
                        </button>
                    }
                    if backpack_wallet.is_some() {
                        <button
                            onclick={connect_backpack_wallet.clone()}
                        >
                            <img
                                src={backpack_wallet_info.icon()}
                                alt="Backpack Wallet"
                            />
                            { "Connect Backpack Wallet" }
                        </button>
                    }
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

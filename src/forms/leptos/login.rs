use leptos::*;

use crate::core::traits::WalletAdapter;
use crate::core::wallet::BaseWalletAdapter;
use crate::pubkey::Pubkey;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn LoginForm(
    phantom: Option<(
        ReadSignal<BaseWalletAdapter>,
        WriteSignal<BaseWalletAdapter>,
    )>,
    solflare: Option<(
        ReadSignal<BaseWalletAdapter>,
        WriteSignal<BaseWalletAdapter>,
    )>,
    backpack: Option<(
        ReadSignal<BaseWalletAdapter>,
        WriteSignal<BaseWalletAdapter>,
    )>,
    connected: (ReadSignal<bool>, WriteSignal<bool>),
) -> impl IntoView {
    let (connected, set_connected) = connected;

    let (mut phantom_wallet_adapter, mut set_phantom_wallet_adapter) =
        create_signal(BaseWalletAdapter::default());
    let (mut solflare_wallet_adapter, mut set_solflare_wallet_adapter) =
        create_signal(BaseWalletAdapter::default());
    let (mut backpack_wallet_adapter, mut set_backpack_wallet_adapter) =
        create_signal(BaseWalletAdapter::default());

    if phantom.is_some() {
        (phantom_wallet_adapter, set_phantom_wallet_adapter) = phantom.unwrap();
    }
    if solflare.is_some() {
        (solflare_wallet_adapter, set_solflare_wallet_adapter) = solflare.unwrap();
    }
    if backpack.is_some() {
        (backpack_wallet_adapter, set_backpack_wallet_adapter) = backpack.clone().unwrap();
    }

    let (error, set_error) = create_signal(String::default());

    let connect_phantom_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = phantom_wallet_adapter.get();

            wallet_info
                .emitter
                .on("connect", move |public_key: Pubkey| {
                    log::info!("Event Listener: Got pubkey {}", public_key);
                });

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(wallet_info);
                    }
                    set_connected.set(confirmed);
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
            let mut wallet_info = solflare_wallet_adapter.get();

            wallet_info
                .emitter
                .on("connect", move |public_key: Pubkey| {
                    log::info!("Event Listener: Got pubkey {}", public_key);
                });

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(wallet_info);
                    }
                    set_connected.set(confirmed);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                    set_error.set(err.to_string());
                }
            }
        });
    };

    let connect_backpack_wallet = move |_| {
        spawn_local(async move {
            let mut wallet_info = backpack_wallet_adapter.get();

            wallet_info
                .emitter
                .on("connect", move |public_key: Pubkey| {
                    log::info!("Event Listener: Got pubkey {}", public_key);
                });

            match wallet_info.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(wallet_info);
                    }
                    set_connected.set(confirmed);
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
            let mut phantom_wallet_info = phantom_wallet_adapter.get();
            let mut solflare_wallet_info = solflare_wallet_adapter.get();
            let mut backpack_wallet_info = backpack_wallet_adapter.get();

            match phantom_wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_phantom_wallet_adapter.set(phantom_wallet_info);
                    }
                    set_connected.set(!confirmed);
                }
                Err(_err) => {}
            }
            match solflare_wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_solflare_wallet_adapter.set(solflare_wallet_info);
                    }
                    set_connected.set(!confirmed);
                }
                Err(_err) => {}
            }
            match backpack_wallet_info.disconnect().await {
                Ok(confirmed) => {
                    if confirmed {
                        set_backpack_wallet_adapter.set(backpack_wallet_info);
                    }
                    set_connected.set(!confirmed);
                }
                Err(_err) => {}
            }
        });
    };

    view! {
        <div class="container">
            <div class="buttons">
                {move ||
                    if !connected.get() {
                        view!{
                            <button class="connect-button-phantom" on:click=connect_phantom_wallet>
                                <img src={phantom_wallet_adapter.get().icon()} alt="Phantom Wallet" class="button-icon" />
                                "Connect Phantom Wallet"
                            </button>
                            <button class="connect-button-solflare" on:click=connect_solflare_wallet>
                                <img src={solflare_wallet_adapter.get().icon()} alt="Solflare Wallet" class="button-icon" />
                                "Connect Solflare Wallet"
                            </button>
                            <button class="connect-button-backpack" on:click=connect_backpack_wallet>
                                <img src={backpack_wallet_adapter.get().icon()} alt="Backpack Wallet" class="button-icon" />
                                "Connect Backpack Wallet"
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
                                <img src={solflare_wallet_adapter.get().icon()} alt="Solflare Wallet" class="button-icon" />
                                "Disconnect Wallet"
                            </button>
                            <>
                            </>
                        }
                    } else if let Some(_key) = backpack_wallet_adapter.get().public_key() {
                        view!{
                            <button class="disconnect-button" on:click=disconnect_wallet>
                                <img src={backpack_wallet_adapter.get().icon()} alt="Backpack Wallet" class="button-icon" />
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
                            <p class="error-message">{ error.get() }</p>
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
    }
}

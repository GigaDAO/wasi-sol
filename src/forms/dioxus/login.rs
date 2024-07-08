use dioxus::prelude::*;
use web_sys::window;

use crate::adapter::{backpack::XNFT, phantom::SOLANA, solflare::SOLFLARE};
use crate::core::traits::WalletAdapter;
use crate::core::wallet::BaseWalletAdapter;

#[derive(Props, Clone, PartialEq)]
pub struct CompProps {
    pub phantom: Option<Signal<BaseWalletAdapter, UnsyncStorage>>,
    pub solflare: Option<Signal<BaseWalletAdapter, UnsyncStorage>>,
    pub backpack: Option<Signal<BaseWalletAdapter, UnsyncStorage>>,
    pub connected: Signal<bool, UnsyncStorage>,
}

#[component]
pub fn LoginForm(props: CompProps) -> Element {
    let mut connected = props.connected;

    let phantom_wallet_adapter = props.phantom;
    let solflare_wallet_adapter = props.solflare;
    let backpack_wallet_adapter = props.backpack;

    let phantom_wallet_info = phantom_wallet_adapter().clone();
    let solflare_wallet_info = solflare_wallet_adapter().clone();
    let backpack_wallet_info = backpack_wallet_adapter().clone();

    let error = use_signal(|| None as Option<String>);

    let connect_wallet_phantom = move |_| {
        let mut phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let phantom_wallet_info = (*phantom_wallet_adapter)().clone();
        let url = phantom_wallet_info.clone().url();

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

        if SOLANA.is_undefined() {
            let window = window().expect("no global `window` exists");
            window
                .open_with_url(&url)
                .expect("failed to open a new tab");
        }
    };

    let connect_wallet_solflare = move |_| {
        let mut solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let solflare_wallet_info = (*solflare_wallet_adapter)().clone();
        let url = solflare_wallet_info.clone().url();

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

        if SOLFLARE.is_undefined() {
            let window = window().expect("no global `window` exists");
            window
                .open_with_url(&url)
                .expect("failed to open a new tab");
        }
    };

    let connect_wallet_backpack = move |_| {
        let mut backpack_wallet_adapter = backpack_wallet_adapter.clone();
        let backpack_wallet_info = (*backpack_wallet_adapter)().clone();
        let url = backpack_wallet_info.clone().url();

        spawn(async move {
            let mut backpack_wallet_info = (*backpack_wallet_adapter)().clone();

            match backpack_wallet_info.connect().await {
                Ok(_) => {
                    backpack_wallet_adapter.set(backpack_wallet_info);
                    connected.set(true);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                }
            }
        });

        if XNFT.is_undefined() {
            let window = window().expect("no global `window` exists");
            window
                .open_with_url(&url)
                .expect("failed to open a new tab");
        }
    };

    let disconnect_wallet = move |_| {
        let mut phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let mut solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let mut backpack_wallet_adapter = backpack_wallet_adapter.clone();

        spawn(async move {
            let mut phantom_wallet_info = (*phantom_wallet_adapter)().clone();
            let mut solflare_wallet_info = (*solflare_wallet_adapter)().clone();
            let mut backpack_wallet_info = (*backpack_wallet_adapter)().clone();

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
    };

    rsx! {
        div {
            class: "container",
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
                        "Connect Phantom Wallet"
                    }
                    button {
                        class: "connect-button-solflare",
                        onclick: connect_wallet_solflare,
                        img {
                            src: solflare_wallet_info.icon(),
                            alt: "Solflare Wallet",
                            class: "button-icon-solflare"
                        },
                        "Connect Solflare Wallet"
                    }
                    button {
                        class: "connect-button-backpack",
                        onclick: connect_wallet_backpack,
                        img {
                            src: backpack_wallet_info.icon(),
                            alt: "Backpack Wallet",
                            class: "button-icon-backpack"
                        },
                        "Connect Backpack Wallet"
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
    }
}

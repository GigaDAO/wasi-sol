use dioxus::prelude::*;

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

    let phantom_wallet = props.phantom;
    let solflare_wallet = props.solflare;
    let backpack_wallet = props.backpack;

    let mut phantom_wallet_info = use_signal(|| BaseWalletAdapter::default());
    let mut solflare_wallet_info = use_signal(|| BaseWalletAdapter::default());
    let mut backpack_wallet_info = use_signal(|| BaseWalletAdapter::default());

    if phantom_wallet.is_some() {
        phantom_wallet_info = phantom_wallet.unwrap();
    }
    if solflare_wallet.is_some() {
        solflare_wallet_info = solflare_wallet.unwrap();
    }
    if backpack_wallet.is_some() {
        backpack_wallet_info = backpack_wallet.unwrap();
    }

    let error = use_signal(|| None as Option<String>);

    let connect_wallet_phantom = move |_| {
        spawn(async move {
            let mut phantom_wallet_adapter = phantom_wallet_info();

            match phantom_wallet_adapter.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        phantom_wallet_info.set(phantom_wallet_adapter);
                    }
                    connected.set(confirmed);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                }
            }
        });
    };

    let connect_wallet_solflare = move |_| {
        spawn(async move {
            let mut solflare_wallet_adapter = solflare_wallet_info();

            match solflare_wallet_adapter.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        solflare_wallet_info.set(solflare_wallet_adapter);
                    }
                    connected.set(confirmed);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                }
            }
        });
    };

    let connect_wallet_backpack = move |_| {
        spawn(async move {
            let mut backpack_wallet_adapter = backpack_wallet_info();

            match backpack_wallet_adapter.connect().await {
                Ok(confirmed) => {
                    if confirmed {
                        backpack_wallet_info.set(backpack_wallet_adapter);
                    }
                    connected.set(confirmed);
                }
                Err(err) => {
                    log::error!("Failed to connect wallet: {}", err);
                }
            }
        });
    };

    let disconnect_wallet = move |_| {
        spawn(async move {
            let mut phantom_wallet_adapter = phantom_wallet_info();
            let mut solflare_wallet_adapter = solflare_wallet_info();
            let mut backpack_wallet_adapter = backpack_wallet_info();

            match phantom_wallet_adapter.disconnect().await {
                Ok(_) => {
                    phantom_wallet_info.set(phantom_wallet_adapter);
                    connected.set(false);
                }
                Err(_err) => {}
            }

            match solflare_wallet_adapter.disconnect().await {
                Ok(_) => {
                    solflare_wallet_info.set(solflare_wallet_adapter);
                    connected.set(false);
                }
                Err(_err) => {}
            }

            match backpack_wallet_adapter.disconnect().await {
                Ok(_) => {
                    backpack_wallet_info.set(backpack_wallet_adapter);
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
                if !connected() {
                    if phantom_wallet.is_some() {
                        button {
                            class: "connect-button-phantom",
                            onclick: connect_wallet_phantom,
                            img {
                                src: phantom_wallet_info().icon(),
                                alt: "Phantom Wallet",
                                class: "button-icon-phantom"
                            },
                            "Connect Phantom Wallet"
                        }
                    }
                    if solflare_wallet.is_some() {
                        button {
                            class: "connect-button-solflare",
                            onclick: connect_wallet_solflare,
                            img {
                                src: solflare_wallet_info().icon(),
                                alt: "Solflare Wallet",
                                class: "button-icon-solflare"
                            },
                            "Connect Solflare Wallet"
                        }
                    }
                    if backpack_wallet.is_some() {
                        button {
                            class: "connect-button-backpack",
                            onclick: connect_wallet_backpack,
                            img {
                                src: backpack_wallet_info().icon(),
                                alt: "Backpack Wallet",
                                class: "button-icon-backpack"
                            },
                            "Connect Backpack Wallet"
                        }
                    }
                } else if let Some(ref _key) = phantom_wallet_info().public_key() {
                    button {
                        class: "disconnect-button",
                        onclick: disconnect_wallet,
                        img {
                            src: phantom_wallet_info().icon(),
                            alt: "Disconnect Wallet",
                            class: "button-icon"
                        },
                        "Disconnect Wallet"
                    }
                    } else if let Some(ref _key) = solflare_wallet_info().public_key() {
                    button {
                        class: "disconnect-button",
                        onclick: disconnect_wallet,
                        img {
                            src: solflare_wallet_info().icon(),
                            alt: "Disconnect Wallet",
                            class: "button-icon"
                        },
                        "Disconnect Wallet"
                    }
                    },
                if let Some(ref e) = error() {
                    p {
                        style: "color: red;",
                        { e.clone() }
                    }
                }
            },
        },
    }
}

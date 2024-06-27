use crate::{
    core::{
        traits::WalletAdapter,
        wallet::{BaseWalletAdapter, Wallet},
    },
    provider::dioxus::local_storage::use_local_storage,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WalletProviderProps {
    pub children: Element,
    pub wallets: Vec<BaseWalletAdapter>,
    #[props(default = "walletName")]
    pub local_storage_key: &'static str,
    #[props(default = false)]
    pub auto_connect: bool,
}

#[component]
pub fn WalletProvider(props: WalletProviderProps) -> Element {
    let (_wallet_name, _set_wallet_name) = use_local_storage(
        props.local_storage_key.to_string(),
        format!("{:?}", Wallet::default()).to_string(),
    );

    let wallet_context = use_memo(move || props.wallets.clone());

    let context = use_signal(move || (*wallet_context)().clone());

    use_context_provider(|| context());

    rsx! { { &props.children } }
}

pub fn use_wallet(wallet_name: Wallet) -> BaseWalletAdapter {
    let wallets = use_context::<Vec<BaseWalletAdapter>>();
    let (_wallet_name, _set_wallet_name) = use_local_storage(
        "walletName".to_string(),
        format!("{:?}", wallet_name).to_string(),
    );
    wallets
        .iter()
        .find(|wallet| wallet.name() == format!("{:?}", wallet_name).to_string())
        .cloned()
        .expect("Wallet not found")
}

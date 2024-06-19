use crate::{
    core::{traits::WalletAdapter, wallet::BaseWalletAdapter},
    provider::dioxus::local_storage::use_local_storage,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WalletProviderProps {
    pub children: Element,
    pub endpoint: &'static str,
    pub wallets: Vec<BaseWalletAdapter>,
    #[props(default = "walletName")]
    pub local_storage_key: &'static str,
    #[props(default = false)]
    pub auto_connect: bool,
}

#[component]
pub fn WalletProvider(props: WalletProviderProps) -> Element {
    let (wallet_name, _set_wallet_name) =
        use_local_storage(props.local_storage_key.to_string(), "Phantom".to_string());

    let wallet_context = use_memo(move || {
        props
            .wallets
            .iter()
            .find(|wallet| wallet.name() == wallet_name)
            .cloned()
    });

    let context = use_signal(move || (*wallet_context)().clone().unwrap());

    use_context_provider(|| context());

    rsx! { { &props.children } }
}

#[component]
pub fn use_wallet() -> BaseWalletAdapter {
    use_context::<BaseWalletAdapter>()
}

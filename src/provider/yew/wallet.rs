use crate::{
    core::{
        traits::WalletAdapter,
        wallet::{BaseWalletAdapter, Wallet},
    },
    provider::yew::local_storage::use_local_storage,
};

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct WalletProviderProps {
    pub children: Children,
    pub wallets: Vec<BaseWalletAdapter>,
    #[prop_or("walletName")]
    pub local_storage_key: &'static str,
    #[prop_or(false)]
    pub auto_connect: bool,
}

#[function_component]
pub fn WalletProvider(props: &WalletProviderProps) -> Html {
    let (wallet_name, _set_wallet_name) = use_local_storage(
        props.local_storage_key.to_string(),
        format!("{:?}", Wallet::default()).to_string(),
    );

    let wallet_context = use_memo((props.wallets.clone(), wallet_name.clone()), |_| {
        props.wallets.clone()
    });

    html! {
        <ContextProvider<Vec<BaseWalletAdapter>> context={(*wallet_context).clone()}>
            { props.children.clone() }
        </ContextProvider<Vec<BaseWalletAdapter>>>
    }
}

#[hook]
pub fn use_wallet<W>(wallet_name: W) -> BaseWalletAdapter
where
    W: Into<BaseWalletAdapter> + std::fmt::Debug,
{
    let wallets = use_context::<Vec<BaseWalletAdapter>>().expect("No WalletContext found");
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

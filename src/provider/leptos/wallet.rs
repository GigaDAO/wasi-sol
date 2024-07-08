use crate::{
    core::{
        traits::WalletAdapter,
        wallet::{BaseWalletAdapter, Wallet},
    },
    provider::leptos::local_storage::use_local_storage,
};
use leptos::*;

#[derive(Clone)]
pub struct Wallets {
    pub wallets: Vec<BaseWalletAdapter>,
}

#[component]
pub fn WalletProvider(
    children: Children,
    wallets: Vec<BaseWalletAdapter>,
    #[prop(default = "walletName")] local_storage_key: &'static str,
) -> impl IntoView {
    let (_wallet_name, _set_wallet_name) = use_local_storage(
        local_storage_key.to_string(),
        format!("{:?}", Wallet::default()).to_string(),
    );

    let wallet_context = create_memo(move |_| wallets.clone());

    view! {
        <Provider<Wallets> value={Wallets { wallets: wallet_context.get_untracked()}}>
           {children()}
        </Provider<Wallets>>
    }
}
pub fn use_wallet<W>(wallet_name: W) -> BaseWalletAdapter
where
    W: Into<BaseWalletAdapter> + std::fmt::Debug,
{
    let wallets = use_context::<Wallets>().expect("No WalletContext found");
    let (_wallet_name, _set_wallet_name) = use_local_storage(
        "walletName".to_string(),
        format!("{:?}", Wallet::default()).to_string(),
    );
    wallets
        .wallets
        .iter()
        .find(|wallet| wallet.name() == format!("{:?}", wallet_name).to_string())
        .cloned()
        .expect("Wallet not found")
}

use crate::{
    core::{traits::WalletAdapter, wallet::BaseWalletAdapter},
    provider::leptos::local_storage::use_local_storage,
};
use leptos::*;

#[component]
pub fn WalletProvider(
    children: Children,
    wallets: Vec<BaseWalletAdapter>,
    #[prop(default = "walletName")] local_storage_key: &'static str,
) -> impl IntoView {
    let (wallet_name, _set_wallet_name) =
        use_local_storage(local_storage_key.to_string(), "Phantom".to_string());

    let wallet_context = create_memo(move |_| {
        wallets
            .iter()
            .find(|wallet| wallet.name() == wallet_name)
            .cloned()
    });

    let (context, _set_context) = create_signal(wallet_context.get_untracked());

    view! {
        <Provider<BaseWalletAdapter> value={context.get_untracked().unwrap()}>
           {children()}
        </Provider<BaseWalletAdapter>>
    }
}

pub fn use_wallet() -> BaseWalletAdapter {
    use_context::<BaseWalletAdapter>().expect("No WalletContext found")
}

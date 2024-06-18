use crate::{
    core::{traits::WalletAdapter, wallet::BaseWalletAdapter},
    provider::local_storage::use_local_storage,
};

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct WalletProviderProps {
    pub children: Children,
    pub endpoint: &'static str,
    pub wallets: Vec<BaseWalletAdapter>,
    #[prop_or("walletName")]
    pub local_storage_key: &'static str,
    #[prop_or(false)]
    pub auto_connect: bool,
}

#[function_component]
pub fn WalletProvider(props: &WalletProviderProps) -> Html {
    // TODO: use this hook to get the endpoint from the WasmClient instance, not available atm, will open a PR
    // Open an Issue/PR to add support for: https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#method.url
    // let connection_context = use_connection();
    // TODO: Add support for multiple wallets
    let (wallet_name, _set_wallet_name) =
        use_local_storage(props.local_storage_key.to_string(), "Phantom".to_string());

    // use_effect_with((), move |_| {
    //     set_wallet_name.emit("Phantom".to_string())
    // });

    let wallet_context = use_memo((props.wallets.clone(), wallet_name.clone()), |_| {
        props
            .wallets
            .iter()
            .find(|wallet| wallet.name() == wallet_name)
            .cloned()
    });

    let context = use_state(|| (*wallet_context).clone().unwrap());

    html! {
        <ContextProvider<BaseWalletAdapter> context={(*context).clone()}>
            { props.children.clone() }
        </ContextProvider<BaseWalletAdapter>>
    }
}

#[hook]
pub fn use_wallet() -> BaseWalletAdapter {
    use_context::<BaseWalletAdapter>().expect("No WalletContext found")
}

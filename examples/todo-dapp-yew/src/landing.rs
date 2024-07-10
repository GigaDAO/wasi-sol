use wasi_sol::{
    core::wallet::Wallet, forms::yew::login::LoginForm, provider::yew::wallet::use_wallet,
    pubkey::Pubkey, spawn_local,
};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub connected: UseStateHandle<bool>,
}

#[function_component]
pub fn Landing(props: &Props) -> Html {
    let phantom_context = use_wallet::<Wallet>(Wallet::Phantom);
    let solflare_context = use_wallet::<Wallet>(Wallet::Solflare);
    let backpack_context = use_wallet::<Wallet>(Wallet::Backpack);

    let phantom_wallet_adapter = use_state(|| phantom_context);
    let solflare_wallet_adapter = use_state(|| solflare_context);
    let backpack_wallet_adapter = use_state(|| backpack_context);

    html! {
        <div class="container">
            if !*props.connected {
                <img class="hero-image" src="./images/logo.jpeg" alt="Hero Image"/>
            }
            <LoginForm
                phantom={Some(phantom_wallet_adapter)}
                solflare={Some(solflare_wallet_adapter)}
                backpack={Some(backpack_wallet_adapter)}
                connected={props.clone().connected}
            />
        </div>
    }
}

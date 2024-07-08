use leptos::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::Wallet,
    forms::leptos::login::LoginForm,
    provider::leptos::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction
};

use std::str::FromStr;

#[component]
pub fn App() -> impl IntoView {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        Wallet::Phantom.into(),
        Wallet::Solflare.into(),
        Wallet::Backpack.into(),
    ];

    view! {
        <ConnectionProvider endpoint=endpoint>
            <WalletProvider wallets=wallets>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let _connection_context = use_connection();
    let phantom_context = use_wallet::<Wallet>(Wallet::Phantom);
    let solflare_context = use_wallet::<Wallet>(Wallet::Solflare);
    let backpack_context = use_wallet::<Wallet>(Wallet::Backpack);
    let (connected, set_connected) = create_signal(false);
    let (phantom_wallet_adapter, set_phantom_wallet_adapter) = create_signal(phantom_context);
    let (solflare_wallet_adapter, set_solflare_wallet_adapter) = create_signal(solflare_context);
    let (backpack_wallet_adapter, set_sbackpack_wallet_adapter) = create_signal(backpack_context);

    let input_dest_ref: NodeRef<html::Input> = create_node_ref();
    let input_amount_ref: NodeRef<html::Input> = create_node_ref();
    let input_msg_ref: NodeRef<html::Input> = create_node_ref();

    let (dest, _set_dest) = create_signal(String::default());
    let (amount, _set_amount) = create_signal(1);
    let (msg, _set_msg) = create_signal(String::default());
    let (sig, set_sig) = create_signal(String::default());
    let (confirmed, set_confirmed) = create_signal(false);

    let transfer_sol_phantom = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
            let input_dest = input_dest_ref.get()
                .expect("<input> should be mounted")
                .value();
            let amount = input_amount_ref.get()
                .expect("<input> should be mounted")
                .value().parse::<u64>().unwrap();
        spawn_local(async move {
            let mut wallet_info = phantom_wallet_adapter.get();
            let public_key = wallet_info.public_key().unwrap();
            let transfer_instruction = system_instruction::transfer(
                &public_key,
                &Pubkey::from_str(&input_dest).unwrap(),
                amount,
            );

            let tx = Transaction::new_with_payer(&[transfer_instruction], Some(&public_key));
            match wallet_info.sign_send_transaction(tx.clone()).await {
                Ok(tx) => {
                    set_sig.set(tx.to_string());
                    set_confirmed.set(true);
                }
                Err(err) => {
                    log::error!("Error: {}", err);
                }
            }
        });
    };

    let sign_msg_phantom = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let mut wallet_info = phantom_wallet_adapter.get();
            let input_msg = input_msg_ref.get()
                .expect("<input> should be mounted")
                .value();

            match wallet_info.sign_message(&input_msg).await {
                Ok(tx) => {
                    set_sig.set(tx.to_string());
                    set_confirmed.set(true);
                }
                Err(err) => {
                    log::error!("Error: {}", err);
                }
            }
        });
    };

    view! {
        <div class="wallet-adapter">
            <header class="header">
                {move ||
                    if !connected.get() {
                        Some(view!{<img src="images/leptos-logo.png" alt="Leptos Logo" class="leptos-logo" />})
                    } else {
                        None
                    }
                }
                <h1>"Wasi Sol Wallet Adapter"</h1>
            </header>
            <div class="content">
                <div class="wallet-info">
                    {move ||
                        if connected.get() {
                            Some(view!{
                                {move ||
                                    if let Some(key) = phantom_wallet_adapter.get().public_key() {
                                        view!{
                                            <p>"Connected Wallet: " {move || phantom_wallet_adapter.get().name()} </p>
                                            <p>"Connected Public Key: " {move || key.to_string() } </p>
                                            <div class="forms">
                                                <div class="send-sol-form">
                                                    <h2 class="form-title">{ "Transfer SOL" }</h2>
                                                    <form on:submit={transfer_sol_phantom}>
                                                        <div class="form-group">
                                                            <label for="destination-address">
                                                                { "Destination Address" }
                                                            </label>
                                                            <input
                                                                id="destination-address"
                                                                type="text"
                                                                class="form-control"
                                                                node_ref={input_dest_ref}
                                                                required=true
                                                                value=dest
                                                            />
                                                        </div>
                                                        <div class="form-group">
                                                            <label for="sol-amount">
                                                                { "SOL Amount (in lamports)" }
                                                            </label>
                                                            <input
                                                                id="sol-amount"
                                                                type="number"
                                                                class="form-control"
                                                                node_ref={input_amount_ref}
                                                                required=true
                                                                value=amount
                                                            />
                                                        </div>
                                                        <button type="submit" class="submit-button">{ "Send" }</button>
                                                    </form>
                                                </div>
                                                <div class="sign-form">
                                                    <h2 class="form-title">{ "Sign Message" }</h2>
                                                    <form on:submit={sign_msg_phantom}>
                                                        <div class="form-group">
                                                            <label for="message">
                                                                { "Message" }
                                                            </label>
                                                            <input
                                                                id="Message"
                                                                type="text"
                                                                class="form-control"
                                                                node_ref={input_msg_ref}
                                                                required=true
                                                                value=msg
                                                            />
                                                        </div>
                                                        <button type="submit" class="submit-button">{ "Sign" }</button>
                                                    </form>
                                                </div>
                                            </div>
                                            {move ||
                                                if confirmed.get() {
                                                    Some(view!{
                                                        <div class="transaction-info">
                                                            <p>{ "Transaction Successful!" }</p>
                                                            <a
                                                                href={format!("https://solscan.io/tx/{}", sig.get())}
                                                                target="_blank"
                                                                rel="noopener noreferrer"
                                                                class="view-transaction-button"
                                                            >
                                                                { "View Transaction" }
                                                            </a>
                                                        </div>
                                                    })
                                                } else {
                                                    None
                                                }
                                            }
                                        }
                                    } else if let Some(key) = solflare_wallet_adapter.get().public_key() {
                                        view!{
                                            <p>"Connected Wallet: " {move || solflare_wallet_adapter.get().name()} </p>
                                            <p>"Connected Public Key: " {move || key.to_string() } </p>
                                        }
                                    } else {
                                        view!{
                                            <p>"Connected but no wallet info available"</p>
                                            <p>{}</p>
                                        }
                                    }
                                }
                            })
                        } else {
                            None
                        }
                    }
                </div>
                <LoginForm
                    phantom=Some((phantom_wallet_adapter, set_phantom_wallet_adapter))
                    solflare=Some((solflare_wallet_adapter, set_solflare_wallet_adapter))
                    backpack=Some((backpack_wallet_adapter, set_sbackpack_wallet_adapter))
                    connected=(connected, set_connected)
                 />
            </div>
            <footer class="footer">
                <p>"2024 GigaDAO Foundation."</p>
            </footer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| view! { <App/> })
}

use yew::prelude::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::Wallet,
    forms::yew::login::LoginForm,
    provider::yew::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    pubkey::Pubkey,
    spawn_local, system_instruction,
    transaction::Transaction,
};

use std::str::FromStr;
use web_sys::HtmlInputElement;

#[function_component]
pub fn App() -> Html {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        Wallet::Phantom.into(),
        Wallet::Solflare.into(),
        Wallet::Backpack.into(),
    ];

    html! {
        <ConnectionProvider {endpoint}>
            <WalletProvider {wallets}>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[function_component]
pub fn LoginPage() -> Html {
    let _connection_context = use_connection();
    let phantom_context = use_wallet::<Wallet>(Wallet::Phantom);
    let solflare_context = use_wallet::<Wallet>(Wallet::Solflare);
    let backpack_context = use_wallet::<Wallet>(Wallet::Backpack);

    let phantom_wallet_adapter = use_state(|| phantom_context);
    let solflare_wallet_adapter = use_state(|| solflare_context);
    let backpack_wallet_adapter = use_state(|| backpack_context);

    let phantom_wallet_info = (*phantom_wallet_adapter).clone();
    let solflare_wallet_info = (*solflare_wallet_adapter).clone();
    let backpack_wallet_info = (*backpack_wallet_adapter).clone();

    let connected = use_state(|| false);
    let confirmed = use_state(|| false);

    let error = use_state(|| None as Option<String>);

    let signature = use_state(String::default);
    let sig = (*signature).clone();

    let input_dest_ref = use_node_ref();
    let input_dest_handle = use_state(String::default);
    let input_dest = (*input_dest_handle).clone();

    let input_amount_ref = use_node_ref();
    let input_amount_handle = use_state(|| 1);
    let input_amount = (*input_amount_handle).clone();

    let input_msg_ref = use_node_ref();
    let input_msg_handle = use_state(String::default);
    let input_msg = (*input_msg_handle).clone();

    let on_dest_change = {
        let input_dest_ref = input_dest_ref.clone();

        Callback::from(move |_| {
            let input = input_dest_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_dest_handle.set(value);
            }
        })
    };

    let on_amount_change = {
        let input_amount_ref = input_amount_ref.clone();

        Callback::from(move |_| {
            let input = input_amount_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_amount_handle.set(value.parse().unwrap());
            }
        })
    };

    let on_msg_change = {
        let input_msg_ref = input_msg_ref.clone();

        Callback::from(move |_| {
            let input = input_msg_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_msg_handle.set(value);
            }
        })
    };

    let transfer_sol_phantom = {
        let phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_dest = input_dest.clone();
        let signature = signature.clone();
        let error = error.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_dest = input_dest.clone();
            let confirmed = confirmed.clone();

            let phantom_wallet_adapter = phantom_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*phantom_wallet_adapter).clone();
                let public_key = wallet_info.public_key().unwrap();

                let transfer_instruction = system_instruction::transfer(
                    &public_key,
                    &Pubkey::from_str(&input_dest).unwrap(),
                    input_amount,
                );

                let tx = Transaction::new_with_payer(&[transfer_instruction], Some(&public_key));
                match wallet_info.sign_send_transaction(tx.clone()).await {
                    Ok(tx) => {
                        signature.set(tx.to_string());
                        confirmed.set(true);
                    }
                    Err(err) => {
                        log::error!("Error: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let sign_msg_phantom = {
        let phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_msg = input_msg.clone();
        let signature = signature.clone();
        let error = error.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_msg = input_msg.clone();
            let confirmed = confirmed.clone();

            let phantom_wallet_adapter = phantom_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*phantom_wallet_adapter).clone();

                match wallet_info.sign_message(&input_msg).await {
                    Ok(tx) => {
                        signature.set(tx.to_string());
                        confirmed.set(true);
                    }
                    Err(err) => {
                        log::error!("Error: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let transfer_sol_solflare = {
        let solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_dest = input_dest.clone();
        let signature = signature.clone();
        let error = error.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_dest = input_dest.clone();
            let confirmed = confirmed.clone();

            let solflare_wallet_adapter = solflare_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*solflare_wallet_adapter).clone();
                let public_key = wallet_info.public_key().unwrap();

                let transfer_instruction = system_instruction::transfer(
                    &public_key,
                    &Pubkey::from_str(&input_dest).unwrap(),
                    input_amount,
                );

                let tx = Transaction::new_with_payer(&[transfer_instruction], Some(&public_key));
                match wallet_info.sign_send_transaction(tx.clone()).await {
                    Ok(tx) => {
                        signature.set(tx.to_string());
                        confirmed.set(true);
                    }
                    Err(err) => {
                        log::error!("Error: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let sign_msg_solflare = {
        let solflare_wallet_adapter = solflare_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_msg = input_msg.clone();
        let error = error.clone();
        let signature = signature.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_msg = input_msg.clone();
            let confirmed = confirmed.clone();

            let solflare_wallet_adapter = solflare_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*solflare_wallet_adapter).clone();

                match wallet_info.sign_message(&input_msg).await {
                    Ok(tx) => {
                        signature.set(tx.to_string());
                        confirmed.set(true);
                    }
                    Err(err) => {
                        log::error!("Error: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let transfer_sol_backpack = {
        let backpack_wallet_adapter = backpack_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_dest = input_dest.clone();
        let signature = signature.clone();
        let error = error.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_dest = input_dest.clone();
            let confirmed = confirmed.clone();

            let backpack_wallet_adapter = backpack_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*backpack_wallet_adapter).clone();
                let public_key = wallet_info.public_key().unwrap();

                let transfer_instruction = system_instruction::transfer(
                    &public_key,
                    &Pubkey::from_str(&input_dest).unwrap(),
                    input_amount,
                );

                let tx = Transaction::new_with_payer(&[transfer_instruction], Some(&public_key));
                match wallet_info.sign_send_transaction(tx.clone()).await {
                    Ok(tx) => {
                        signature.set(tx.to_string());
                        confirmed.set(true);
                    }
                    Err(err) => {
                        log::error!("Error: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let sign_msg_backpack = {
        let backpack_wallet_adapter = backpack_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_msg = input_msg.clone();
        let signature = signature.clone();
        let error = error.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_msg = input_msg.clone();
            let confirmed = confirmed.clone();

            let backpack_wallet_adapter = backpack_wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*backpack_wallet_adapter).clone();

                match wallet_info.sign_message(&input_msg).await {
                    Ok(tx) => {
                        signature.set(tx.to_string());
                        confirmed.set(true);
                    }
                    Err(err) => {
                        log::error!("Error: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    html! {
        <div class="wallet-adapter">
            <header class="header">
                if !*connected {
                    <img src="images/logo.jpeg" alt="Yew Logo" class="yew-logo" />
                }
                <h1>{ "Wasi Sol Yew Wallet Adapter" }</h1>
            </header>
            <div class="content">
                <div class="wallet-info">
                    if *connected {
                        if let Some(ref key) = phantom_wallet_info.public_key() {
                            <p>{ format!("Connected Wallet: {}", phantom_wallet_info.name()) }</p>
                            <p>{ format!("Connected Public Key: {}", key) }</p>
                            <div class="forms">
                                <div class="send-sol-form">
                                    <h2 class="form-title">{ "Transfer SOL" }</h2>
                                    <form onsubmit={transfer_sol_phantom}>
                                        <div class="form-group">
                                            <label for="destination-address">
                                                { "Destination Address" }
                                            </label>
                                            <input
                                                id="destination-address"
                                                type="text"
                                                class="form-control"
                                                ref={input_dest_ref}
                                                required=true
                                                oninput={on_dest_change}
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
                                                ref={input_amount_ref}
                                                required=true
                                                oninput={on_amount_change}
                                            />
                                        </div>
                                        <button type="submit" class="submit-button">{ "Send" }</button>
                                    </form>
                                </div>
                                <div class="sign-form">
                                    <h2 class="form-title">{ "Sign Message" }</h2>
                                    <form onsubmit={sign_msg_phantom}>
                                        <div class="form-group">
                                            <label for="message">
                                                { "Message" }
                                            </label>
                                            <input
                                                id="Message"
                                                type="text"
                                                class="form-control"
                                                ref={input_msg_ref}
                                                required=true
                                                oninput={on_msg_change}
                                            />
                                        </div>
                                        <button type="submit" class="submit-button">{ "Sign" }</button>
                                    </form>
                                </div>
                            </div>
                            if *confirmed {
                                <div class="transaction-info">
                                    <p>{ "Transaction Successful!" }</p>
                                    <a
                                        href={format!("https://solscan.io/tx/{}", sig)}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="view-transaction-button"
                                    >
                                        { "View Transaction" }
                                    </a>
                                </div>
                            }
                        } else if let Some(ref key) = solflare_wallet_info.public_key() {
                            <p>{ format!("Connected Wallet: {}", solflare_wallet_info.name()) }</p>
                            <p>{ format!("Connected Public Key: {}", key) }</p>
                            <div class="forms">
                                <div class="send-sol-form">
                                    <h2 class="form-title">{ "Transfer SOL" }</h2>
                                    <form onsubmit={transfer_sol_solflare}>
                                        <div class="form-group">
                                            <label for="destination-address">
                                                { "Destination Address" }
                                            </label>
                                            <input
                                                id="destination-address"
                                                type="text"
                                                class="form-control"
                                                ref={input_dest_ref}
                                                required=true
                                                oninput={on_dest_change}
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
                                                ref={input_amount_ref}
                                                required=true
                                                oninput={on_amount_change}
                                            />
                                        </div>
                                        <button type="submit" class="submit-button">{ "Send" }</button>
                                    </form>
                                </div>
                                <div class="sign-form">
                                    <h2 class="form-title">{ "Sign Message" }</h2>
                                    <form onsubmit={sign_msg_solflare}>
                                        <div class="form-group">
                                            <label for="message">
                                                { "Message" }
                                            </label>
                                            <input
                                                id="Message"
                                                type="text"
                                                class="form-control"
                                                ref={input_msg_ref}
                                                required=true
                                                oninput={on_msg_change}
                                            />
                                        </div>
                                        <button type="submit" class="submit-button">{ "Sign" }</button>
                                    </form>
                                </div>
                            </div>
                            if *confirmed {
                                <div class="transaction-info">
                                    <p>{ "Transaction Successful!" }</p>
                                    <a
                                        href={format!("https://solscan.io/tx/{}", sig)}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="view-transaction-button"
                                    >
                                        { "View Transaction" }
                                    </a>
                                </div>
                            }
                        } else if let Some(ref key) = backpack_wallet_info.public_key() {
                            <p>{ format!("Connected Wallet: {}", backpack_wallet_info.name()) }</p>
                            <p>{ format!("Connected Public Key: {}", key) }</p>
                            <div class="forms">
                                <div class="send-sol-form">
                                    <h2 class="form-title">{ "Transfer SOL (Coming Soon)" }</h2>
                                    <form onsubmit={transfer_sol_backpack}>
                                        <div class="form-group">
                                            <label for="destination-address">
                                                { "Destination Address" }
                                            </label>
                                            <input
                                                id="destination-address"
                                                type="text"
                                                class="form-control"
                                                ref={input_dest_ref}
                                                required=true
                                                oninput={on_dest_change}
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
                                                ref={input_amount_ref}
                                                required=true
                                                oninput={on_amount_change}
                                            />
                                        </div>
                                        <button type="submit" class="submit-button">{ "Send" }</button>
                                    </form>
                                </div>
                                <div class="sign-form">
                                    <h2 class="form-title">{ "Sign Message (Coming Soon)" }</h2>
                                    <form onsubmit={sign_msg_backpack}>
                                        <div class="form-group">
                                            <label for="message">
                                                { "Message" }
                                            </label>
                                            <input
                                                id="Message"
                                                type="text"
                                                class="form-control"
                                                ref={input_msg_ref}
                                                required=true
                                                oninput={on_msg_change}
                                            />
                                        </div>
                                        <button type="submit" class="submit-button">{ "Sign" }</button>
                                    </form>
                                </div>
                            </div>
                            if *confirmed {
                                <div class="transaction-info">
                                    <p>{ "Transaction Successful!" }</p>
                                    <a
                                        href={format!("https://solscan.io/tx/{}", sig)}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="view-transaction-button"
                                    >
                                        { "View Transaction" }
                                    </a>
                                </div>
                            }
                        } else {
                            <p>{ "Connected but no wallet info available" }</p>
                        }
                    }
                </div>
                <LoginForm
                    phantom={Some(phantom_wallet_adapter)}
                    solflare={Some(solflare_wallet_adapter)}
                    backpack={Some(backpack_wallet_adapter)}
                    {connected}
                />
                if let Some(ref e) = *error {
                    <p style="color: red;">{ e.clone() }</p>
                }
            </div>
            <footer class="footer">
                <p>{ "2024 GigaDAO Foundation." }</p>
            </footer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::transaction::TransactionOrVersionedTransaction,
    core::wallet::{BaseWalletAdapter, Wallet},
    forms::yew::login::LoginForm,
    provider::yew::{
        connection::{use_connection, ConnectionProvider},
        wallet::{use_wallet, WalletProvider},
    },
    pubkey::Pubkey,
    signer::keypair::Keypair,
    spawn_local, system_instruction,
    transaction::Transaction,
};

use std::str::FromStr;
use web_sys::HtmlInputElement;

#[function_component]
pub fn App() -> Html {
    // Use custom rpc endpoint;
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        BaseWalletAdapter::new(
            Wallet::Solflare,
            "https://solflare.com",
            "images/solflare_logo.png",
        ),
        BaseWalletAdapter::new(
            Wallet::Phantom,
            "https://phantom.app",
            "images/phantom_logo.png",
        ),
        BaseWalletAdapter::new(
            Wallet::Backpack,
            "https://backpack.app",
            "images/backpack_logo.png",
        ),
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
    let connection_context = use_connection();
    let phantom_context = use_wallet(Wallet::Phantom);
    let solflare_context = use_wallet(Wallet::Solflare);
    let backpack_context = use_wallet(Wallet::Backpack);

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

    let input_secret_ref = use_node_ref();
    let input_secret_handle = use_state(String::default);
    let input_secret = (*input_secret_handle).clone();

    let input_dest_ref = use_node_ref();
    let input_dest_handle = use_state(String::default);
    let input_dest = (*input_dest_handle).clone();

    let input_amount_ref = use_node_ref();
    let input_amount_handle = use_state(|| 1);
    let input_amount = (*input_amount_handle).clone();

    let on_secret_change = {
        let input_secret_ref = input_secret_ref.clone();

        Callback::from(move |_| {
            let input = input_secret_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_secret_handle.set(value);
            }
        })
    };

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

    let transfer_sol = {
        let phantom_wallet_adapter = phantom_wallet_adapter.clone();
        let confirmed = confirmed.clone();
        let input_secret = input_secret.clone();
        let input_dest = input_dest.clone();
        let error = error.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let signature = signature.clone();
            let input_secret = input_secret.clone();
            let input_dest = input_dest.clone();
            let confirmed = confirmed.clone();

            let phantom_wallet_adapter = phantom_wallet_adapter.clone();
            let connection_context = connection_context.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*phantom_wallet_adapter).clone();
                let public_key = wallet_info.public_key().unwrap();
                let client = &connection_context.connection;

                let transfer_instruction = system_instruction::transfer(
                    &public_key,
                    &Pubkey::from_str(&input_dest).unwrap(),
                    input_amount,
                );
                let recent_blockhash = client.get_latest_blockhash().await.unwrap();

                let keypair = Keypair::from_base58_string(&input_secret);

                let tx = Transaction::new_signed_with_payer(
                    &[transfer_instruction],
                    Some(&public_key),
                    &[&keypair],
                    recent_blockhash,
                );

                let transaction = TransactionOrVersionedTransaction::Transaction(tx);

                match wallet_info
                    .send_transaction(client.clone(), transaction)
                    .await
                {
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
                            <div class="send-sol-form">
                                <h2 class="form-title">{ "Transfer SOL" }</h2>
                                <form onsubmit={transfer_sol}>
                                    <div class="form-group">
                                        <label for="secret-key">{ "Secret Key" }</label>
                                        <input
                                            id="secret-key"
                                            type="password"
                                            class="form-control"
                                            ref={input_secret_ref}
                                            required=true
                                            oninput={on_secret_change}
                                        />
                                    </div>
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
                        } else if let Some(ref key) = backpack_wallet_info.public_key() {
                            <p>{ format!("Connected Wallet: {}", backpack_wallet_info.name()) }</p>
                            <p>{ format!("Connected Public Key: {}", key) }</p>
                        } else {
                            <p>{ "Connected but no wallet info available" }</p>
                        }
                    }
                </div>
                <LoginForm
                    phantom={phantom_wallet_adapter}
                    solflare={solflare_wallet_adapter}
                    backpack={backpack_wallet_adapter}
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

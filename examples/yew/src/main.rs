use yew::prelude::*;

use wasi_sol::{
    core::traits::WalletAdapter,
    core::transaction::TransactionOrVersionedTransaction,
    core::wallet::BaseWalletAdapter,
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
    // Use helius for guaranteed transaction success;
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![BaseWalletAdapter::new(
        "Phantom",
        "https://phantom.app",
        "images/phantom_logo.png",
    )];

    html! {
        <ConnectionProvider {endpoint}>
            <WalletProvider {endpoint} {wallets}>
                <LoginPage />
            </WalletProvider>
        </ConnectionProvider>
    }
}

#[function_component]
pub fn LoginPage() -> Html {
    let connection_context = use_connection();
    let wallet_context = use_wallet();
    let connected = use_state(|| false);
    let confirmed = use_state(|| false);
    let wallet_adapter = use_state(|| wallet_context);

    let wallet_info = (*wallet_adapter).clone();
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

    let connect_wallet = {
        let connected = connected.clone();
        let wallet_adapter = wallet_adapter.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let wallet_adapter = wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

                wallet_info
                    .emitter
                    .on("connect", move |public_key: Pubkey| {
                        log::info!("Event Listener: Got pubkey {}", public_key);
                    });

                match wallet_info.connect().await {
                    Ok(_) => {
                        wallet_adapter.set(wallet_info);
                        connected.set(true);
                    }
                    Err(err) => {
                        log::error!("Failed to connect wallet: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let disconnect_wallet = {
        let connected = connected.clone();
        let wallet_adapter = wallet_adapter.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let connected = connected.clone();
            let wallet_adapter = wallet_adapter.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();

                match wallet_info.disconnect().await {
                    Ok(_) => {
                        wallet_adapter.set(wallet_info);
                        connected.set(false);
                    }
                    Err(err) => {
                        log::error!("Failed to disconnect wallet: {}", err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

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
        let wallet_adapter = wallet_adapter.clone();
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

            let wallet_adapter = wallet_adapter.clone();
            let connection_context = connection_context.clone();
            let error = error.clone();

            spawn_local(async move {
                let mut wallet_info = (*wallet_adapter).clone();
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
                        if let Some(ref key) = wallet_info.public_key() {
                            <p>{ format!("Connected Wallet: {}", wallet_info.name()) }</p>
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
                        } else {
                            <p>{ "Connected but no wallet info available" }</p>
                        }
                    }
                </div>
                <div class="buttons">
                    if !*connected {
                        <button class="connect-button" onclick={connect_wallet}>
                            <img src={wallet_info.icon()} alt="Phantom Wallet" class="button-icon" />
                            { "Connect Wallet" }
                        </button>
                    } else {
                        <button class="disconnect-button" onclick={disconnect_wallet}>
                            <img
                                src={wallet_info.icon()}
                                alt="Disconnect Wallet"
                                class="button-icon"
                            />
                            { "Disconnect Wallet" }
                        </button>
                    }
                </div>
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

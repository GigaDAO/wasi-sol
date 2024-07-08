use dioxus::prelude::*;
use std::str::FromStr;
use wasi_sol::{
    core::traits::WalletAdapter,
    core::wallet::Wallet,
    forms::dioxus::login::LoginForm,
    provider::dioxus::{
        connection::ConnectionProvider,
        wallet::{use_wallet, WalletProvider},
    },
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    launch(app);
}

fn app() -> Element {
    let endpoint = "https://api.mainnet-beta.solana.com";
    let wallets = vec![
        Wallet::Solflare.into(),
        Wallet::Phantom.into(),
        Wallet::Backpack.into(),
    ];

    rsx! {
        ConnectionProvider {
            endpoint: endpoint,
            WalletProvider {
                wallets: wallets,
                LoginPage {}
            }
        }
    }
}

#[component]
fn LoginPage() -> Element {
    let phantom_context = use_wallet(Wallet::Phantom);
    let solflare_context = use_wallet(Wallet::Solflare);
    let backpack_context = use_wallet(Wallet::Backpack);

    let phantom_wallet_adapter = use_signal(|| phantom_context);
    let solflare_wallet_adapter = use_signal(|| solflare_context);
    let backpack_wallet_adapter = use_signal(|| backpack_context);

    let connected = use_signal(|| false);
    let mut error = use_signal(|| Some(String::default()));
    let mut confirmed = use_signal(|| false);
    let mut signature = use_signal(|| String::default());

    let mut input_dest = use_signal(|| String::default());
    let mut input_amount = use_signal(|| String::default());
    let mut input_msg = use_signal(|| String::default());

    let transfer_sol_phantom = move |event: Event<FormData>| {
        event.stop_propagation();
        spawn(async move {
            let mut wallet_info = phantom_wallet_adapter();
            let public_key = wallet_info.public_key().unwrap();
            let transfer_instruction = system_instruction::transfer(
                &public_key,
                &Pubkey::from_str(&input_dest()).unwrap(),
                input_amount().parse::<u64>().unwrap(),
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
    };

    let sign_msg_phantom = move |event: Event<FormData>| {
        event.stop_propagation();
        spawn(async move {
            let mut wallet_info = phantom_wallet_adapter();
            match wallet_info.sign_message(&input_msg()).await {
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
    };

    let transfer_sol_solflare = move |event: Event<FormData>| {
        event.stop_propagation();
        spawn(async move {
            let mut wallet_info = solflare_wallet_adapter();
            let public_key = wallet_info.public_key().unwrap();
            let transfer_instruction = system_instruction::transfer(
                &public_key,
                &Pubkey::from_str(&input_dest()).unwrap(),
                input_amount().parse::<u64>().unwrap(),
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
    };

    let sign_msg_solflare = move |event: Event<FormData>| {
        event.stop_propagation();
        spawn(async move {
            let mut wallet_info = solflare_wallet_adapter();
            match wallet_info.sign_message(&input_msg()).await {
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
    };

    let transfer_sol_backpack = move |event: Event<FormData>| {
        event.stop_propagation();
        spawn(async move {
            let mut wallet_info = backpack_wallet_adapter();
            let public_key = wallet_info.public_key().unwrap();
            let transfer_instruction = system_instruction::transfer(
                &public_key,
                &Pubkey::from_str(&input_dest()).unwrap(),
                input_amount().parse::<u64>().unwrap(),
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
    };

    let sign_msg_backpack = move |event: Event<FormData>| {
        event.stop_propagation();
        spawn(async move {
            let mut wallet_info = backpack_wallet_adapter();
            match wallet_info.sign_message(&input_msg()).await {
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
    };

    rsx! {
        div {
            class: "wallet-adapter",
            header {
                class: "header",
                img {
                    src: "./header.svg",
                    alt: "Phantom Wallet",
                    class: "button-icon"
                },
                h1 { "Wasi Sol Dioxus Wallet Adapter" }
            },
            div {
                class: "content",
                div {
                    class: "wallet-info",
                    if connected() {
                        if let Some(ref key) = phantom_wallet_adapter().public_key() {
                            p { "Connected Wallet: {phantom_wallet_adapter().name()}" }
                            p { "Connected Public Key: {key}" }
                            div {
                                class: "forms",
                                div {
                                    class: "send-sol-form",
                                    h2 {
                                        class: "form-title",
                                        "Transfer SOL"
                                    }
                                    form {
                                        onsubmit: transfer_sol_phantom,
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "destination-address",
                                                "Destination Address"
                                            }
                                            input {
                                                id: "destination-address",
                                                r#type: "text",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_dest}",
                                                oninput: move |evt| input_dest.set(evt.value().clone()),
                                            }
                                        }
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "sol-amount",
                                                "SOL Amount (in lamports)"
                                            }
                                            input {
                                                id: "sol-amount",
                                                r#type: "number",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_amount}",
                                                oninput: move |evt| input_amount.set(evt.value().clone()),
                                            }
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "submit-button",
                                            "Send"
                                        }
                                    }
                                }
                                div {
                                    class: "sign-form",
                                    h2 {
                                        class: "form-title",
                                        "Sign Message"
                                    }
                                    form {
                                        onsubmit: sign_msg_phantom,
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "message",
                                                "Message"
                                            }
                                            input {
                                                id: "message",
                                                r#type: "text",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_msg}",
                                                oninput: move |evt| input_msg.set(evt.value().clone()),
                                            }
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "submit-button",
                                            "Sign"
                                        }
                                    }
                                }
                            }
                            if confirmed() {
                                div {
                                    class: "transaction-info",
                                    p { "Transaction Successful!" }
                                    a {
                                        href: format!("https://solscan.io/tx/{}", signature()),
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "view-transaction-button",
                                        "View Transaction"
                                    }
                                }
                            }
                        } else if let Some(ref key) = solflare_wallet_adapter().public_key() {
                            p { "Connected Wallet: {solflare_wallet_adapter().name()}" }
                            p { "Connected Public Key: {key}" }
                            div {
                                class: "forms",
                                div {
                                    class: "send-sol-form",
                                    h2 {
                                        class: "form-title",
                                        "Transfer SOL"
                                    }
                                    form {
                                        onsubmit: transfer_sol_solflare,
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "destination-address",
                                                "Destination Address"
                                            }
                                            input {
                                                id: "destination-address",
                                                r#type: "text",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_dest}",
                                                oninput: move |evt| input_dest.set(evt.value().clone()),
                                            }
                                        }
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "sol-amount",
                                                "SOL Amount (in lamports)"
                                            }
                                            input {
                                                id: "sol-amount",
                                                r#type: "number",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_amount}",
                                                oninput: move |evt| input_amount.set(evt.value().clone()),
                                            }
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "submit-button",
                                            "Send"
                                        }
                                    }
                                }
                                div {
                                    class: "sign-form",
                                    h2 {
                                        class: "form-title",
                                        "Sign Message"
                                    }
                                    form {
                                        onsubmit: sign_msg_solflare,
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "message",
                                                "Message"
                                            }
                                            input {
                                                id: "message",
                                                r#type: "text",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_msg}",
                                                oninput: move |evt| input_msg.set(evt.value().clone()),
                                            }
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "submit-button",
                                            "Sign"
                                        }
                                    }
                                }
                            }
                            if confirmed() {
                                div {
                                    class: "transaction-info",
                                    p { "Transaction Successful!" }
                                    a {
                                        href: format!("https://solscan.io/tx/{}", signature()),
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "view-transaction-button",
                                        "View Transaction"
                                    }
                                }
                            }
                        } else if let Some(ref key) = backpack_wallet_adapter().public_key() {
                            p { "Connected Wallet: {solflare_wallet_adapter().name()}" }
                            p { "Connected Public Key: {key}" }
                            div {
                                class: "forms",
                                div {
                                    class: "send-sol-form",
                                    h2 {
                                        class: "form-title",
                                        "Transfer SOL (Coming Soon)"
                                    }
                                    form {
                                        onsubmit: transfer_sol_backpack,
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "destination-address",
                                                "Destination Address"
                                            }
                                            input {
                                                id: "destination-address",
                                                r#type: "text",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_dest}",
                                                oninput: move |evt| input_dest.set(evt.value().clone()),
                                            }
                                        }
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "sol-amount",
                                                "SOL Amount (in lamports)"
                                            }
                                            input {
                                                id: "sol-amount",
                                                r#type: "number",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_amount}",
                                                oninput: move |evt| input_amount.set(evt.value().clone()),
                                            }
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "submit-button",
                                            "Send"
                                        }
                                    }
                                }
                                div {
                                    class: "sign-form",
                                    h2 {
                                        class: "form-title",
                                        "Sign Message (Coming Soon)"
                                    }
                                    form {
                                        onsubmit: sign_msg_backpack,
                                        div {
                                            class: "form-group",
                                            label {
                                                r#for: "message",
                                                "Message"
                                            }
                                            input {
                                                id: "message",
                                                r#type: "text",
                                                class: "form-control",
                                                required: true,
                                                value: "{input_msg}",
                                                oninput: move |evt| input_msg.set(evt.value().clone()),
                                            }
                                        }
                                        button {
                                            r#type: "submit",
                                            class: "submit-button",
                                            "Sign"
                                        }
                                    }
                                }
                            }
                            if confirmed() {
                                div {
                                    class: "transaction-info",
                                    p { "Transaction Successful!" }
                                    a {
                                        href: format!("https://solscan.io/tx/{}", signature()),
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        class: "view-transaction-button",
                                        "View Transaction"
                                    }
                                }
                            }
                        } else {
                            p { "Connected but no wallet info available" }
                        }
                    }
                },
            },
            LoginForm {
                phantom: Some(phantom_wallet_adapter),
                solflare: Some(solflare_wallet_adapter),
                backpack: Some(backpack_wallet_adapter),
                connected: connected
            }
            footer {
                class: "footer",
                p { "2024 GigaDAO Foundation." }
            }
        }
    }
}

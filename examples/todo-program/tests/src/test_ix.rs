use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster, Program,
};
use std::str::FromStr;
use std::sync::Arc;
use todo_program::{
    self, accounts,
    constants::{TODO_STATE_SEED, USER_STATE_SEED},
    instruction,
};

const PROGRAM_ID: &str = "A7LQKx8zxi4smRhViytbVTksJyR1WnLqKUqua5h3SXQA";

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Pubkey, Keypair) {
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(program_id).unwrap();

    (client, program, program_id, payer.insecure_clone())
}

#[test]
fn test_initialize_account() {
    let (_client, program, program_id, authority) = setup_program();

    let todo_account_pubkey =
        Pubkey::find_program_address(&[USER_STATE_SEED, authority.pubkey().as_ref()], &program_id)
            .0;

    let tx = program
        .request()
        .accounts(accounts::InitializeAccount {
            authority: authority.pubkey(),
            todo_account: todo_account_pubkey,
            system_program: system_program::ID,
        })
        .args(instruction::InitializeAccount {})
        .send()
        .expect("Failed to send initialize account transaction");

    println!("InitializeAccount transaction signature: {}", tx);
}

#[test]
fn test_create_todo() {
    let (_client, program, program_id, authority) = setup_program();

    let todo_account_pubkey =
        Pubkey::find_program_address(&[USER_STATE_SEED, authority.pubkey().as_ref()], &program_id)
            .0;

    let todo_id: u128 = 1;
    let todo_user_pubkey = Pubkey::find_program_address(
        &[
            TODO_STATE_SEED,
            authority.pubkey().as_ref(),
            &todo_id.to_le_bytes(),
        ],
        &program_id,
    )
    .0;

    let tx = program
        .request()
        .accounts(accounts::CreateTodo {
            authority: authority.pubkey(),
            todo_account: todo_account_pubkey,
            todo_user: todo_user_pubkey,
            system_program: system_program::ID,
        })
        .args(instruction::CreateTodo {
            todo_id: todo_id,
            content: String::from("Test Todo"),
        })
        .send()
        .expect("Failed to send create todo transaction");

    println!("CreateTodo transaction signature: {}", tx);
}

#[test]
fn test_update_todo() {
    test_create_todo();

    let (_client, program, program_id, authority) = setup_program();

    let todo_account_pubkey =
        Pubkey::find_program_address(&[USER_STATE_SEED, authority.pubkey().as_ref()], &program_id)
            .0;

    let todo_id: u128 = 1;
    let todo_user_pubkey = Pubkey::find_program_address(
        &[
            TODO_STATE_SEED,
            authority.pubkey().as_ref(),
            &todo_id.to_le_bytes(),
        ],
        &program_id,
    )
    .0;

    let tx = program
        .request()
        .accounts(accounts::UpdateTodo {
            authority: authority.pubkey(),
            todo_account: todo_account_pubkey,
            todo_user: todo_user_pubkey,
            system_program: system_program::ID,
        })
        .args(instruction::UpdateTodo {
            todo_id: todo_id,
            content: Some(String::from("Updated Test Todo")),
            completed: Some(true),
        })
        .send()
        .expect("Failed to send update todo transaction");

    println!("UpdateTodo transaction signature: {}", tx);
}

#[test]
fn test_delete_todo() {
    test_initialize_account();
    test_create_todo();

    let (_client, program, program_id, authority) = setup_program();

    let todo_account_pubkey =
        Pubkey::find_program_address(&[USER_STATE_SEED, authority.pubkey().as_ref()], &program_id)
            .0;

    let todo_id: u128 = 1;
    let todo_user_pubkey = Pubkey::find_program_address(
        &[
            TODO_STATE_SEED,
            authority.pubkey().as_ref(),
            &todo_id.to_le_bytes(),
        ],
        &program_id,
    )
    .0;

    let tx = program
        .request()
        .accounts(accounts::DeleteTodo {
            authority: authority.pubkey(),
            todo_account: todo_account_pubkey,
            todo_user: todo_user_pubkey,
            system_program: system_program::ID,
        })
        .args(instruction::DeleteTodo { todo_id: todo_id })
        .send()
        .expect("Failed to send delete todo transaction");

    println!("DeleteTodo transaction signature: {}", tx);
}

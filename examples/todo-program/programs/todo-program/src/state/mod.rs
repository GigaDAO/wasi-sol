use crate::{TODO_STATE_SEED, USER_STATE_SEED};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
/// The main account for a user, holding the user ID and the authority (owner) of the account.
pub struct TodoAccount {
    /// The public key of the authority (owner) of the account.
    pub authority: Pubkey,
    /// The ID for the todo list, which is a unique identifier.
    pub todo_id: u128,
}

#[account]
#[derive(Default)]
/// Represents a single todo item for a user.
pub struct TodoUser {
    /// The ID of the todo item.
    pub id: u128,
    /// The public key of the authority (owner) of the todo item.
    pub authority: Pubkey,
    /// The content or description of the todo item.
    pub content: String,
    /// A boolean indicating whether the todo item is completed.
    pub completed: bool,
}

#[derive(Accounts)]
#[instruction()]
/// Context for initializing a new TodoAccount.
pub struct InitializeAccount<'info> {
    /// The account of the user who is initializing the todo account.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The todo account to be initialized, derived from USER_STATE_SEED and the authority's public key.
    #[account(
        init,
        seeds = [USER_STATE_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<TodoAccount>(),
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    /// The system program, which is required for account creation.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_id: u128)]
/// Context for creating a new todo item.
pub struct CreateTodo<'info> {
    /// The todo account of the user creating the todo item, which must already exist.
    #[account(
        mut,
        seeds = [USER_STATE_SEED, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    /// The todo item to be created, derived from TODO_STATE_SEED, the authority's public key, and the todo ID.
    #[account(
        init,
        seeds = [TODO_STATE_SEED, authority.key().as_ref(), &todo_id.to_le_bytes()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<TodoUser>(),
    )]
    pub todo_user: Box<Account<'info, TodoUser>>,

    /// The account of the user creating the todo item.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The system program, which is required for account creation.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_id: u128)]
/// Context for updating an existing todo item.
pub struct UpdateTodo<'info> {
    /// The todo account of the user updating the todo item, which must already exist.
    #[account(
        mut,
        seeds = [USER_STATE_SEED, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    /// The todo item to be updated, derived from TODO_STATE_SEED, the authority's public key, and the todo ID.
    #[account(
        mut,
        seeds = [TODO_STATE_SEED, authority.key().as_ref(), &todo_id.to_le_bytes()],
        bump,
        has_one = authority,
    )]
    pub todo_user: Box<Account<'info, TodoUser>>,

    /// The account of the user updating the todo item.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The system program, which is required for account operations.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_id: u128)]
/// Context for deleting an existing todo item.
pub struct DeleteTodo<'info> {
    /// The todo account of the user deleting the todo item, which must already exist.
    #[account(
        mut,
        seeds = [USER_STATE_SEED, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    /// The todo item to be deleted, derived from TODO_STATE_SEED, the authority's public key, and the todo ID.
    #[account(
        mut,
        close = authority,
        seeds = [TODO_STATE_SEED, authority.key().as_ref(), &todo_id.to_le_bytes()],
        bump,
        has_one = authority,
    )]
    pub todo_user: Box<Account<'info, TodoUser>>,

    /// The account of the user deleting the todo item.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The system program, which is required for account operations.
    pub system_program: Program<'info, System>,
}

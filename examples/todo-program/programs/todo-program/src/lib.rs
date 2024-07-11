pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("A7LQKx8zxi4smRhViytbVTksJyR1WnLqKUqua5h3SXQA");

#[program]
pub mod todo_program {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        initialize::account(ctx)
    }

    pub fn create_todo(ctx: Context<CreateTodo>, content: String, todo_id: u128) -> Result<()> {
        create::todo(ctx, content, todo_id)
    }

    pub fn update_todo(
        ctx: Context<UpdateTodo>,
        todo_id: u128,
        content: Option<String>,
        completed: Option<bool>,
    ) -> Result<()> {
        update::todo(ctx, todo_id, content, completed)
    }

    pub fn delete_todo(ctx: Context<DeleteTodo>, todo_id: u128) -> Result<()> {
        delete::todo(ctx, todo_id)
    }
}

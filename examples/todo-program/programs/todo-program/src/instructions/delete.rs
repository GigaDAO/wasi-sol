use crate::error::TodoError;
use crate::DeleteTodo;
use anchor_lang::prelude::*;

pub fn todo(ctx: Context<DeleteTodo>, todo_id: u128) -> Result<()> {
    let todo_account = &mut ctx.accounts.todo_account;

    require!(todo_account.todo_id == todo_id, TodoError::InvalidTodoId);

    Ok(())
}

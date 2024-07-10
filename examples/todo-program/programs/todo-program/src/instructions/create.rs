use crate::error::TodoError;
use crate::CreateTodo;
use anchor_lang::prelude::*;

pub fn todo(ctx: Context<CreateTodo>, content: String, todo_id: u128) -> Result<()> {
    require!(content.len() <= 280, TodoError::ContentTooLong);

    let todo_user = &mut ctx.accounts.todo_user;
    todo_user.id = todo_id;
    todo_user.authority = *ctx.accounts.authority.key;
    todo_user.content = content;
    todo_user.completed = false;

    Ok(())
}

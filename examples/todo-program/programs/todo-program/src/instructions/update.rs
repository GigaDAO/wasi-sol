use crate::error::TodoError;
use crate::UpdateTodo;
use anchor_lang::prelude::*;

pub fn todo(
    ctx: Context<UpdateTodo>,
    todo_id: u128,
    content: Option<String>,
    completed: Option<bool>,
) -> Result<()> {
    let todo_user = &mut ctx.accounts.todo_user;
    require!(todo_user.id == todo_id, TodoError::InvalidTodoId);

    if let Some(content) = content {
        require!(content.len() <= 280, TodoError::ContentTooLong);
        todo_user.content = content;
    }

    if let Some(completed) = completed {
        todo_user.completed = completed;
    }

    Ok(())
}

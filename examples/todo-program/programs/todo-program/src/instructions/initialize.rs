use crate::InitializeAccount;
use anchor_lang::prelude::*;

pub fn account(ctx: Context<InitializeAccount>) -> Result<()> {
    let todo_account = &mut ctx.accounts.todo_account;
    todo_account.authority = ctx.accounts.authority.key();
    todo_account.todo_id = 0;

    Ok(())
}

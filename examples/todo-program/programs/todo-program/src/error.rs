use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {
    #[msg("The provided content exceeds the maximum allowed length of 280 characters.")]
    ContentTooLong,
    #[msg("The provided Todo ID does not match the existing Todo ID.")]
    InvalidTodoId,
}

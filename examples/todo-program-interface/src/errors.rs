use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum TodoProgramError {
    #[error(
        "The provided content exceeds the maximum allowed length of 280 characters."
    )]
    ContentTooLong = 6000,
    #[error("The provided Todo ID does not match the existing Todo ID.")]
    InvalidTodoId = 6001,
}
impl From<TodoProgramError> for ProgramError {
    fn from(e: TodoProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for TodoProgramError {
    fn type_of() -> &'static str {
        "TodoProgramError"
    }
}
impl PrintProgramError for TodoProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}

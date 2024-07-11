use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum TodoProgramProgramIx {
    InitializeAccount,
    CreateTodo(CreateTodoIxArgs),
    UpdateTodo(UpdateTodoIxArgs),
    DeleteTodo(DeleteTodoIxArgs),
}
impl TodoProgramProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INITIALIZE_ACCOUNT_IX_DISCM => Ok(Self::InitializeAccount),
            CREATE_TODO_IX_DISCM => {
                Ok(Self::CreateTodo(CreateTodoIxArgs::deserialize(&mut reader)?))
            }
            UPDATE_TODO_IX_DISCM => {
                Ok(Self::UpdateTodo(UpdateTodoIxArgs::deserialize(&mut reader)?))
            }
            DELETE_TODO_IX_DISCM => {
                Ok(Self::DeleteTodo(DeleteTodoIxArgs::deserialize(&mut reader)?))
            }
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeAccount => writer.write_all(&INITIALIZE_ACCOUNT_IX_DISCM),
            Self::CreateTodo(args) => {
                writer.write_all(&CREATE_TODO_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateTodo(args) => {
                writer.write_all(&UPDATE_TODO_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DeleteTodo(args) => {
                writer.write_all(&DELETE_TODO_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccountAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub todo_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeAccountKeys {
    pub authority: Pubkey,
    pub todo_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccountAccounts<'_, '_>> for InitializeAccountKeys {
    fn from(accounts: InitializeAccountAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            todo_account: *accounts.todo_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeAccountKeys> for [AccountMeta; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.todo_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]> for InitializeAccountKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            todo_account: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeAccountAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.todo_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]>
for InitializeAccountAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            todo_account: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const INITIALIZE_ACCOUNT_IX_DISCM: [u8; 8] = [74, 115, 99, 93, 197, 69, 103, 7];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeAccountIxData;
impl InitializeAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_ACCOUNT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_ACCOUNT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_ACCOUNT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeAccountKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeAccountIxData.try_to_vec()?,
    })
}
pub fn initialize_account_ix(
    keys: InitializeAccountKeys,
) -> std::io::Result<Instruction> {
    initialize_account_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeAccountKeys = accounts.into();
    let ix = initialize_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_account_invoke(
    accounts: InitializeAccountAccounts<'_, '_>,
) -> ProgramResult {
    initialize_account_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeAccountKeys = accounts.into();
    let ix = initialize_account_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_account_invoke_signed(
    accounts: InitializeAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_account_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn initialize_account_verify_account_keys(
    accounts: InitializeAccountAccounts<'_, '_>,
    keys: InitializeAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.todo_account.key, keys.todo_account),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_account_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.todo_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_account_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_account_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_account_verify_writable_privileges(accounts)?;
    initialize_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TODO_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct CreateTodoAccounts<'me, 'info> {
    pub todo_account: &'me AccountInfo<'info>,
    pub todo_user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTodoKeys {
    pub todo_account: Pubkey,
    pub todo_user: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTodoAccounts<'_, '_>> for CreateTodoKeys {
    fn from(accounts: CreateTodoAccounts) -> Self {
        Self {
            todo_account: *accounts.todo_account.key,
            todo_user: *accounts.todo_user.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTodoKeys> for [AccountMeta; CREATE_TODO_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTodoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.todo_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.todo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_TODO_IX_ACCOUNTS_LEN]> for CreateTodoKeys {
    fn from(pubkeys: [Pubkey; CREATE_TODO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            todo_account: pubkeys[0],
            todo_user: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<CreateTodoAccounts<'_, 'info>>
for [AccountInfo<'info>; CREATE_TODO_IX_ACCOUNTS_LEN] {
    fn from(accounts: CreateTodoAccounts<'_, 'info>) -> Self {
        [
            accounts.todo_account.clone(),
            accounts.todo_user.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TODO_IX_ACCOUNTS_LEN]>
for CreateTodoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TODO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            todo_account: &arr[0],
            todo_user: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const CREATE_TODO_IX_DISCM: [u8; 8] = [250, 161, 142, 148, 131, 48, 194, 181];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTodoIxArgs {
    pub content: String,
    pub todo_id: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTodoIxData(pub CreateTodoIxArgs);
impl From<CreateTodoIxArgs> for CreateTodoIxData {
    fn from(args: CreateTodoIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTodoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TODO_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CREATE_TODO_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CreateTodoIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TODO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_todo_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTodoKeys,
    args: CreateTodoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TODO_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTodoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_todo_ix(
    keys: CreateTodoKeys,
    args: CreateTodoIxArgs,
) -> std::io::Result<Instruction> {
    create_todo_ix_with_program_id(crate::ID, keys, args)
}
pub fn create_todo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTodoAccounts<'_, '_>,
    args: CreateTodoIxArgs,
) -> ProgramResult {
    let keys: CreateTodoKeys = accounts.into();
    let ix = create_todo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_todo_invoke(
    accounts: CreateTodoAccounts<'_, '_>,
    args: CreateTodoIxArgs,
) -> ProgramResult {
    create_todo_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_todo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTodoAccounts<'_, '_>,
    args: CreateTodoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTodoKeys = accounts.into();
    let ix = create_todo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_todo_invoke_signed(
    accounts: CreateTodoAccounts<'_, '_>,
    args: CreateTodoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_todo_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_todo_verify_account_keys(
    accounts: CreateTodoAccounts<'_, '_>,
    keys: CreateTodoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.todo_account.key, keys.todo_account),
        (*accounts.todo_user.key, keys.todo_user),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_todo_verify_writable_privileges<'me, 'info>(
    accounts: CreateTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.todo_account,
        accounts.todo_user,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_todo_verify_signer_privileges<'me, 'info>(
    accounts: CreateTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_todo_verify_account_privileges<'me, 'info>(
    accounts: CreateTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_todo_verify_writable_privileges(accounts)?;
    create_todo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_TODO_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateTodoAccounts<'me, 'info> {
    pub todo_account: &'me AccountInfo<'info>,
    pub todo_user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateTodoKeys {
    pub todo_account: Pubkey,
    pub todo_user: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<UpdateTodoAccounts<'_, '_>> for UpdateTodoKeys {
    fn from(accounts: UpdateTodoAccounts) -> Self {
        Self {
            todo_account: *accounts.todo_account.key,
            todo_user: *accounts.todo_user.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<UpdateTodoKeys> for [AccountMeta; UPDATE_TODO_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateTodoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.todo_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.todo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_TODO_IX_ACCOUNTS_LEN]> for UpdateTodoKeys {
    fn from(pubkeys: [Pubkey; UPDATE_TODO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            todo_account: pubkeys[0],
            todo_user: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateTodoAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_TODO_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateTodoAccounts<'_, 'info>) -> Self {
        [
            accounts.todo_account.clone(),
            accounts.todo_user.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_TODO_IX_ACCOUNTS_LEN]>
for UpdateTodoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_TODO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            todo_account: &arr[0],
            todo_user: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const UPDATE_TODO_IX_DISCM: [u8; 8] = [105, 8, 31, 183, 159, 73, 203, 134];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateTodoIxArgs {
    pub todo_id: u128,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateTodoIxData(pub UpdateTodoIxArgs);
impl From<UpdateTodoIxArgs> for UpdateTodoIxData {
    fn from(args: UpdateTodoIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateTodoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_TODO_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_TODO_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateTodoIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_TODO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_todo_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateTodoKeys,
    args: UpdateTodoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_TODO_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateTodoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_todo_ix(
    keys: UpdateTodoKeys,
    args: UpdateTodoIxArgs,
) -> std::io::Result<Instruction> {
    update_todo_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_todo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTodoAccounts<'_, '_>,
    args: UpdateTodoIxArgs,
) -> ProgramResult {
    let keys: UpdateTodoKeys = accounts.into();
    let ix = update_todo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_todo_invoke(
    accounts: UpdateTodoAccounts<'_, '_>,
    args: UpdateTodoIxArgs,
) -> ProgramResult {
    update_todo_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_todo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateTodoAccounts<'_, '_>,
    args: UpdateTodoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateTodoKeys = accounts.into();
    let ix = update_todo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_todo_invoke_signed(
    accounts: UpdateTodoAccounts<'_, '_>,
    args: UpdateTodoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_todo_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_todo_verify_account_keys(
    accounts: UpdateTodoAccounts<'_, '_>,
    keys: UpdateTodoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.todo_account.key, keys.todo_account),
        (*accounts.todo_user.key, keys.todo_user),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_todo_verify_writable_privileges<'me, 'info>(
    accounts: UpdateTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.todo_account,
        accounts.todo_user,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_todo_verify_signer_privileges<'me, 'info>(
    accounts: UpdateTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_todo_verify_account_privileges<'me, 'info>(
    accounts: UpdateTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_todo_verify_writable_privileges(accounts)?;
    update_todo_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_TODO_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct DeleteTodoAccounts<'me, 'info> {
    pub todo_account: &'me AccountInfo<'info>,
    pub todo_user: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteTodoKeys {
    pub todo_account: Pubkey,
    pub todo_user: Pubkey,
    pub authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<DeleteTodoAccounts<'_, '_>> for DeleteTodoKeys {
    fn from(accounts: DeleteTodoAccounts) -> Self {
        Self {
            todo_account: *accounts.todo_account.key,
            todo_user: *accounts.todo_user.key,
            authority: *accounts.authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DeleteTodoKeys> for [AccountMeta; DELETE_TODO_IX_ACCOUNTS_LEN] {
    fn from(keys: DeleteTodoKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.todo_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.todo_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELETE_TODO_IX_ACCOUNTS_LEN]> for DeleteTodoKeys {
    fn from(pubkeys: [Pubkey; DELETE_TODO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            todo_account: pubkeys[0],
            todo_user: pubkeys[1],
            authority: pubkeys[2],
            system_program: pubkeys[3],
        }
    }
}
impl<'info> From<DeleteTodoAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_TODO_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeleteTodoAccounts<'_, 'info>) -> Self {
        [
            accounts.todo_account.clone(),
            accounts.todo_user.clone(),
            accounts.authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELETE_TODO_IX_ACCOUNTS_LEN]>
for DeleteTodoAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DELETE_TODO_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            todo_account: &arr[0],
            todo_user: &arr[1],
            authority: &arr[2],
            system_program: &arr[3],
        }
    }
}
pub const DELETE_TODO_IX_DISCM: [u8; 8] = [224, 212, 234, 177, 90, 57, 219, 115];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteTodoIxArgs {
    pub todo_id: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTodoIxData(pub DeleteTodoIxArgs);
impl From<DeleteTodoIxArgs> for DeleteTodoIxData {
    fn from(args: DeleteTodoIxArgs) -> Self {
        Self(args)
    }
}
impl DeleteTodoIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_TODO_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DELETE_TODO_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DeleteTodoIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_TODO_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_todo_ix_with_program_id(
    program_id: Pubkey,
    keys: DeleteTodoKeys,
    args: DeleteTodoIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_TODO_IX_ACCOUNTS_LEN] = keys.into();
    let data: DeleteTodoIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delete_todo_ix(
    keys: DeleteTodoKeys,
    args: DeleteTodoIxArgs,
) -> std::io::Result<Instruction> {
    delete_todo_ix_with_program_id(crate::ID, keys, args)
}
pub fn delete_todo_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeleteTodoAccounts<'_, '_>,
    args: DeleteTodoIxArgs,
) -> ProgramResult {
    let keys: DeleteTodoKeys = accounts.into();
    let ix = delete_todo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_todo_invoke(
    accounts: DeleteTodoAccounts<'_, '_>,
    args: DeleteTodoIxArgs,
) -> ProgramResult {
    delete_todo_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn delete_todo_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeleteTodoAccounts<'_, '_>,
    args: DeleteTodoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeleteTodoKeys = accounts.into();
    let ix = delete_todo_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_todo_invoke_signed(
    accounts: DeleteTodoAccounts<'_, '_>,
    args: DeleteTodoIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_todo_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn delete_todo_verify_account_keys(
    accounts: DeleteTodoAccounts<'_, '_>,
    keys: DeleteTodoKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.todo_account.key, keys.todo_account),
        (*accounts.todo_user.key, keys.todo_user),
        (*accounts.authority.key, keys.authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_todo_verify_writable_privileges<'me, 'info>(
    accounts: DeleteTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.todo_account,
        accounts.todo_user,
        accounts.authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_todo_verify_signer_privileges<'me, 'info>(
    accounts: DeleteTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_todo_verify_account_privileges<'me, 'info>(
    accounts: DeleteTodoAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_todo_verify_writable_privileges(accounts)?;
    delete_todo_verify_signer_privileges(accounts)?;
    Ok(())
}

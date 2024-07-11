use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const TODO_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [31, 86, 84, 40, 187, 31, 251, 132];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TodoAccount {
    pub authority: Pubkey,
    pub todo_id: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TodoAccountAccount(pub TodoAccount);
impl TodoAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TODO_ACCOUNT_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TODO_ACCOUNT_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TodoAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TODO_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TODO_USER_ACCOUNT_DISCM: [u8; 8] = [232, 149, 142, 184, 207, 149, 241, 40];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TodoUser {
    pub id: u128,
    pub authority: Pubkey,
    pub content: String,
    pub completed: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TodoUserAccount(pub TodoUser);
impl TodoUserAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TODO_USER_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TODO_USER_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TodoUser::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TODO_USER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

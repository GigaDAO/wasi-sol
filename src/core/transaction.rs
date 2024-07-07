use serde::{Deserialize, Serialize};
use solana_sdk::transaction::{Transaction, VersionedTransaction};

#[derive(Clone, Serialize, Deserialize)]
pub enum TransactionOrVersionedTransaction {
    Transaction(Transaction),
    VersionedTransaction(VersionedTransaction),
}

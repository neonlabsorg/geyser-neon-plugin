use serde::{Deserialize, Serialize};
use solana_account_decoder::parse_token::UiTokenAmount;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    ReplicaAccountInfo, ReplicaAccountInfoV2, ReplicaAccountInfoVersions,
};
use solana_program::hash::Hash;
use solana_program::message::legacy::Message as LegacyMessage;
use solana_program::message::v0::{LoadedAddresses, Message};
use solana_sdk::transaction::Result as TransactionResult;
use solana_sdk::transaction_context::TransactionReturnData;
use solana_sdk::{clock::UnixTimestamp, signature::Signature};
use solana_transaction_status::Rewards;
use solana_transaction_status::{InnerInstructions, Reward};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Information about an account being updated
pub struct NeonReplicaAccountInfo {
    /// The Pubkey for the account
    pub pubkey: Vec<u8>,

    /// The lamports for the account
    pub lamports: u64,

    /// The Pubkey of the owner program account
    pub owner: Vec<u8>,

    /// This account's data contains a loaded program (and is now read-only)
    pub executable: bool,

    /// The epoch at which this account will next owe rent
    pub rent_epoch: u64,

    /// The data held in this account.
    pub data: Vec<u8>,

    /// A global monotonically increasing atomic number, which can be used
    /// to tell the order of the account update. For example, when an
    /// account is updated in the same slot multiple times, the update
    /// with higher write_version should supersede the one with lower
    /// write_version.
    pub write_version: u64,
}

impl From<&ReplicaAccountInfo<'_>> for NeonReplicaAccountInfo {
    fn from(account_info: &ReplicaAccountInfo<'_>) -> Self {
        NeonReplicaAccountInfo {
            pubkey: account_info.pubkey.to_vec(),
            lamports: account_info.lamports,
            owner: account_info.owner.to_vec(),
            executable: account_info.executable,
            rent_epoch: account_info.rent_epoch,
            data: account_info.data.to_vec(),
            write_version: account_info.write_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Information about an account being updated
/// (extended with transaction signature doing this update)
pub struct NeonReplicaAccountInfoV2 {
    /// The Pubkey for the account
    pub pubkey: Vec<u8>,

    /// The lamports for the account
    pub lamports: u64,

    /// The Pubkey of the owner program account
    pub owner: Vec<u8>,

    /// This account's data contains a loaded program (and is now read-only)
    pub executable: bool,

    /// The epoch at which this account will next owe rent
    pub rent_epoch: u64,

    /// The data held in this account.
    pub data: Vec<u8>,

    /// A global monotonically increasing atomic number, which can be used
    /// to tell the order of the account update. For example, when an
    /// account is updated in the same slot multiple times, the update
    /// with higher write_version should supersede the one with lower
    /// write_version.
    pub write_version: u64,

    /// First signature of the transaction caused this account modification
    pub txn_signature: Option<Signature>,
}

impl From<&ReplicaAccountInfoV2<'_>> for NeonReplicaAccountInfoV2 {
    fn from(account_info: &ReplicaAccountInfoV2<'_>) -> Self {
        NeonReplicaAccountInfoV2 {
            pubkey: account_info.pubkey.to_vec(),
            lamports: account_info.lamports,
            owner: account_info.owner.to_vec(),
            executable: account_info.executable,
            rent_epoch: account_info.rent_epoch,
            data: account_info.data.to_vec(),
            write_version: account_info.write_version,
            txn_signature: account_info.txn_signature.copied(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum NeonReplicaTransactionInfoVersions {
    V0_0_1(NeonReplicaTransactionInfo),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NeonSanitizedTransaction {
    message: NeonSanitizedMessage,
    message_hash: Hash,
    is_simple_vote_tx: bool,
    signatures: Vec<Signature>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeonSanitizedMessage {
    /// Sanitized legacy message
    Legacy(LegacyMessage),
    /// Sanitized version #0 message with dynamically loaded addresses
    V0(LoadedMessage),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadedMessage {
    /// Message which loaded a collection of lookup table addresses
    pub message: Message,
    /// Addresses loaded with on-chain address lookup tables
    pub loaded_addresses: LoadedAddresses,
}

/// Information about a transaction
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NeonReplicaTransactionInfo {
    /// The first signature of the transaction, used for identifying the transaction.
    pub signature: Signature,

    /// Indicates if the transaction is a simple vote transaction.
    pub is_vote: bool,

    /// The sanitized transaction.
    pub transaction: NeonSanitizedTransaction,

    /// Metadata of the transaction status.
    pub transaction_status_meta: NeonTransactionStatusMeta,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeonTransactionTokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub ui_token_amount: UiTokenAmount,
    pub owner: String,
    pub program_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeonTransactionStatusMeta {
    pub status: TransactionResult<()>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub inner_instructions: Option<Vec<InnerInstructions>>,
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<NeonTransactionTokenBalance>>,
    pub post_token_balances: Option<Vec<NeonTransactionTokenBalance>>,
    pub rewards: Option<Rewards>,
    pub loaded_addresses: LoadedAddresses,
    pub return_data: Option<TransactionReturnData>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeonReplicaAccountInfoVersions {
    V0_0_1(NeonReplicaAccountInfo),
    V0_0_2(NeonReplicaAccountInfoV2),
}

impl From<ReplicaAccountInfoVersions<'_>> for NeonReplicaAccountInfoVersions {
    fn from(account_info: ReplicaAccountInfoVersions) -> Self {
        match account_info {
            ReplicaAccountInfoVersions::V0_0_1(a) => {
                NeonReplicaAccountInfoVersions::V0_0_1(a.into())
            }
            ReplicaAccountInfoVersions::V0_0_2(a) => {
                NeonReplicaAccountInfoVersions::V0_0_2(a.into())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeonReplicaBlockInfoVersions {
    V0_0_1(NeonReplicaBlockInfo),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeonReplicaBlockInfo {
    pub slot: u64,
    pub blockhash: String,
    pub rewards: Reward,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccount {
    pub account: NeonReplicaAccountInfoVersions,
    pub slot: u64,
    pub is_startup: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSlotStatus {
    pub slot: u64,
    pub parent: Option<u64>,
    pub status: NeonSlotStatus,
}

/// The current status of a slot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeonSlotStatus {
    /// The highest slot of the heaviest fork processed by the node. Ledger state at this slot is
    /// not derived from a confirmed or finalized block, but if multiple forks are present, is from
    /// the fork the validator believes is most likely to finalize.
    Processed,

    /// The highest slot having reached max vote lockout.
    Rooted,

    /// The highest slot that has been voted on by supermajority of the cluster, ie. is confirmed.
    Confirmed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NotifyTransaction {
    pub transaction_info: NeonReplicaTransactionInfoVersions,
    pub slot: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyBlockMetaData {
    pub block_info: NeonReplicaBlockInfoVersions,
}

use serde::{Deserialize, Serialize};
use solana_account_decoder::parse_token::UiTokenAmount;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    ReplicaAccountInfo, ReplicaAccountInfoV2, ReplicaAccountInfoVersions, ReplicaBlockInfo,
    ReplicaBlockInfoVersions, ReplicaTransactionInfo, ReplicaTransactionInfoV2,
    ReplicaTransactionInfoVersions, SlotStatus,
};
use solana_program::hash::Hash;
use solana_program::message::legacy::Message as LegacyMessage;
use solana_program::message::v0::{LoadedAddresses, LoadedMessage, Message};
use solana_program::message::SanitizedMessage;
use solana_sdk::transaction::{Result as TransactionResult, SanitizedTransaction};
use solana_sdk::transaction_context::TransactionReturnData;
use solana_sdk::{clock::UnixTimestamp, signature::Signature};
use solana_transaction_status::{InnerInstructions, Reward};
use solana_transaction_status::{Rewards, TransactionStatusMeta, TransactionTokenBalance};

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
    V0_0_2(NeonReplicaTransactionInfoV2),
}

impl From<&ReplicaTransactionInfoVersions<'_>> for NeonReplicaTransactionInfoVersions {
    fn from(replica_account_info: &ReplicaTransactionInfoVersions<'_>) -> Self {
        match replica_account_info {
            ReplicaTransactionInfoVersions::V0_0_1(t) => {
                NeonReplicaTransactionInfoVersions::V0_0_1(t.into())
            }
            ReplicaTransactionInfoVersions::V0_0_2(t) => {
                NeonReplicaTransactionInfoVersions::V0_0_2(t.into())
            }
        }
    }
}

impl From<ReplicaTransactionInfoVersions<'_>> for NeonReplicaTransactionInfoVersions {
    fn from(replica_account_info: ReplicaTransactionInfoVersions<'_>) -> Self {
        match replica_account_info {
            ReplicaTransactionInfoVersions::V0_0_1(t) => {
                NeonReplicaTransactionInfoVersions::V0_0_1(t.into())
            }
            ReplicaTransactionInfoVersions::V0_0_2(t) => {
                NeonReplicaTransactionInfoVersions::V0_0_2(t.into())
            }
        }
    }
}

impl From<&ReplicaTransactionInfo<'_>> for NeonReplicaTransactionInfo {
    fn from(transaction_info: &ReplicaTransactionInfo<'_>) -> Self {
        NeonReplicaTransactionInfo {
            signature: *transaction_info.signature,
            is_vote: transaction_info.is_vote,
            transaction: transaction_info.transaction.into(),
            transaction_status_meta: transaction_info.transaction_status_meta.into(),
        }
    }
}

impl From<&&ReplicaTransactionInfo<'_>> for NeonReplicaTransactionInfo {
    fn from(transaction_info: &&ReplicaTransactionInfo<'_>) -> Self {
        NeonReplicaTransactionInfo {
            signature: *transaction_info.signature,
            is_vote: transaction_info.is_vote,
            transaction: transaction_info.transaction.into(),
            transaction_status_meta: transaction_info.transaction_status_meta.into(),
        }
    }
}

impl From<&ReplicaTransactionInfoV2<'_>> for NeonReplicaTransactionInfoV2 {
    fn from(transaction_info: &ReplicaTransactionInfoV2<'_>) -> Self {
        NeonReplicaTransactionInfoV2 {
            signature: *transaction_info.signature,
            is_vote: transaction_info.is_vote,
            transaction: transaction_info.transaction.into(),
            transaction_status_meta: transaction_info.transaction_status_meta.into(),
            index: transaction_info.index,
        }
    }
}

impl From<&&ReplicaTransactionInfoV2<'_>> for NeonReplicaTransactionInfoV2 {
    fn from(transaction_info: &&ReplicaTransactionInfoV2<'_>) -> Self {
        NeonReplicaTransactionInfoV2 {
            signature: *transaction_info.signature,
            is_vote: transaction_info.is_vote,
            transaction: transaction_info.transaction.into(),
            transaction_status_meta: transaction_info.transaction_status_meta.into(),
            index: transaction_info.index,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NeonSanitizedTransaction {
    message: NeonSanitizedMessage,
    message_hash: Hash,
    is_simple_vote_tx: bool,
    signatures: Vec<Signature>,
}

impl From<&SanitizedTransaction> for NeonSanitizedTransaction {
    fn from(sanitized_transaction: &SanitizedTransaction) -> Self {
        NeonSanitizedTransaction {
            message: sanitized_transaction.message().into(),
            message_hash: *sanitized_transaction.message_hash(),
            is_simple_vote_tx: sanitized_transaction.is_simple_vote_transaction(),
            signatures: sanitized_transaction.signatures().to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeonSanitizedMessage {
    /// Sanitized legacy message
    Legacy(LegacyMessage),
    /// Sanitized version #0 message with dynamically loaded addresses
    V0(NeonLoadedMessage),
}

impl From<&SanitizedMessage> for NeonSanitizedMessage {
    fn from(sanitized_message: &SanitizedMessage) -> Self {
        match sanitized_message {
            SanitizedMessage::Legacy(sm) => NeonSanitizedMessage::Legacy(sm.to_owned()),
            SanitizedMessage::V0(sm) => NeonSanitizedMessage::V0(sm.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeonLoadedMessage {
    /// Message which loaded a collection of lookup table addresses
    pub message: Message,
    /// Addresses loaded with on-chain address lookup tables
    pub loaded_addresses: LoadedAddresses,
}

impl From<&LoadedMessage<'_>> for NeonLoadedMessage {
    fn from(loaded_message: &LoadedMessage) -> Self {
        NeonLoadedMessage {
            message: loaded_message.message.clone().into_owned(),
            loaded_addresses: loaded_message.loaded_addresses.clone().into_owned(),
        }
    }
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

/// Information about a transaction, including index in block
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NeonReplicaTransactionInfoV2 {
    /// The first signature of the transaction, used for identifying the transaction.
    pub signature: Signature,

    /// Indicates if the transaction is a simple vote transaction.
    pub is_vote: bool,

    /// The sanitized transaction.
    pub transaction: NeonSanitizedTransaction,

    /// Metadata of the transaction status.
    pub transaction_status_meta: NeonTransactionStatusMeta,

    /// The transaction's index in the block
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeonTransactionTokenBalance {
    pub account_index: u8,
    pub mint: String,
    pub ui_token_amount: UiTokenAmount,
    pub owner: String,
    pub program_id: String,
}

impl From<TransactionTokenBalance> for NeonTransactionTokenBalance {
    fn from(transaction_token_balance: TransactionTokenBalance) -> Self {
        NeonTransactionTokenBalance {
            account_index: transaction_token_balance.account_index,
            mint: transaction_token_balance.mint,
            ui_token_amount: transaction_token_balance.ui_token_amount,
            owner: transaction_token_balance.owner,
            program_id: transaction_token_balance.program_id,
        }
    }
}

impl From<&TransactionTokenBalance> for NeonTransactionTokenBalance {
    fn from(transaction_token_balance: &TransactionTokenBalance) -> Self {
        NeonTransactionTokenBalance {
            account_index: transaction_token_balance.account_index,
            mint: transaction_token_balance.mint.clone(),
            ui_token_amount: transaction_token_balance.ui_token_amount.clone(),
            owner: transaction_token_balance.owner.clone(),
            program_id: transaction_token_balance.program_id.clone(),
        }
    }
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

impl From<&TransactionStatusMeta> for NeonTransactionStatusMeta {
    fn from(transaction_status_meta: &TransactionStatusMeta) -> Self {
        let pre_token_balances: Option<Vec<NeonTransactionTokenBalance>> = transaction_status_meta
            .pre_token_balances
            .as_ref()
            .map(|v| {
                let mut result: Vec<NeonTransactionTokenBalance> = Vec::new();
                for i in v {
                    result.push(i.into())
                }
                result
            });

        let post_token_balances: Option<Vec<NeonTransactionTokenBalance>> = transaction_status_meta
            .post_token_balances
            .as_ref()
            .map(|v| {
                let mut result: Vec<NeonTransactionTokenBalance> = Vec::new();
                for i in v {
                    result.push(i.into())
                }
                result
            });

        NeonTransactionStatusMeta {
            status: transaction_status_meta.status.clone(),
            fee: transaction_status_meta.fee,
            pre_balances: transaction_status_meta.pre_balances.clone(),
            post_balances: transaction_status_meta.post_balances.clone(),
            inner_instructions: transaction_status_meta.inner_instructions.clone(),
            log_messages: transaction_status_meta.log_messages.clone(),
            pre_token_balances,
            post_token_balances,
            rewards: transaction_status_meta.rewards.clone(),
            loaded_addresses: transaction_status_meta.loaded_addresses.clone(),
            return_data: transaction_status_meta.return_data.clone(),
        }
    }
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

impl From<ReplicaBlockInfoVersions<'_>> for NeonReplicaBlockInfoVersions {
    fn from(replica_block_info: ReplicaBlockInfoVersions) -> Self {
        match replica_block_info {
            ReplicaBlockInfoVersions::V0_0_1(r) => NeonReplicaBlockInfoVersions::V0_0_1(r.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeonReplicaBlockInfo {
    pub slot: u64,
    pub blockhash: String,
    pub rewards: Vec<Reward>,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

impl From<&ReplicaBlockInfo<'_>> for NeonReplicaBlockInfo {
    fn from(replica_block_info: &ReplicaBlockInfo) -> Self {
        NeonReplicaBlockInfo {
            slot: replica_block_info.slot,
            blockhash: replica_block_info.blockhash.to_string(),
            rewards: replica_block_info.rewards.to_vec(),
            block_time: replica_block_info.block_time,
            block_height: replica_block_info.block_height,
        }
    }
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

impl From<SlotStatus> for NeonSlotStatus {
    fn from(slot_status: SlotStatus) -> Self {
        match slot_status {
            SlotStatus::Processed => NeonSlotStatus::Processed,
            SlotStatus::Rooted => NeonSlotStatus::Rooted,
            SlotStatus::Confirmed => NeonSlotStatus::Confirmed,
        }
    }
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

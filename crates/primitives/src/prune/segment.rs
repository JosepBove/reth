use crate::MINIMUM_PRUNING_DISTANCE;
use derive_more::Display;
use reth_codecs::{main_codec, Compact};
use thiserror::Error;

/// Segment of the data that can be pruned.
#[main_codec]
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PruneSegment {
    /// Prune segment responsible for the `TransactionSenders` table.
    SenderRecovery,
    /// Prune segment responsible for the `TransactionHashNumbers` table.
    TransactionLookup,
    /// Prune segment responsible for all rows in `Receipts` table.
    Receipts,
    /// Prune segment responsible for some rows in `Receipts` table filtered by logs.
    ContractLogs,
    /// Prune segment responsible for the `AccountChangeSes` and `AccountsHistory` tables.
    AccountsHistory,
    /// Prune segment responsible for the `StorageChangeSets` and `StoragesHistory` tables.
    StoragesHistory,
    /// Prune segment responsible for the `CanonicalHeaders`, `Headers` and
    /// `HeaderTerminalDifficulties` tables.
    Headers,
    /// Prune segment responsible for the `Transactions` table.
    Transactions,
}

impl PruneSegment {
    /// Returns minimum number of blocks to left in the database for this segment.
    pub fn min_blocks(&self) -> u64 {
        match self {
            Self::SenderRecovery | Self::TransactionLookup | Self::Headers | Self::Transactions => {
                0
            }
            Self::Receipts | Self::ContractLogs | Self::AccountsHistory | Self::StoragesHistory => {
                MINIMUM_PRUNING_DISTANCE
            }
        }
    }
}

/// PruneSegment error type.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PruneSegmentError {
    /// Invalid configuration of a prune segment.
    #[error("the configuration provided for {0} is invalid")]
    Configuration(PruneSegment),
    /// Receipts have been pruned
    #[error("receipts have been pruned")]
    ReceiptsPruned,
}

#[cfg(test)]
impl Default for PruneSegment {
    fn default() -> Self {
        Self::SenderRecovery
    }
}

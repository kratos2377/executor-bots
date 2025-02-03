use solana_sdk::{instruction::InstructionError, transaction::TransactionError};
use thiserror::Error;
use tokio::sync::oneshot;
use crate::vortex_idl::errors::ErrorCode;

pub type UnsubHandle = oneshot::Sender<()>;

pub type VortexSdkResult<T> = Result<T, VortexSdkError>;

#[derive(Debug, Error)]
pub enum VortexSdkError {
    #[error("{0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
    #[error("invalid URL")]
    InvalidUrl,
    #[error("invalid keypair seed")]
    InvalidSeed,
    #[error("insufficient SOL balance for fees")]
    OutOfSOL,
    #[error("Error while making versioned transaction")]
    ErrorWhileMakingVersionTransaction,
    #[error("Error while parsing transaction record")]
    ErrorWhileParsingTransactionRecord
}


impl VortexSdkError {
        /// extract anchor error code from the VortexSdkError if it exists
        pub fn to_anchor_error_code(&self) -> Option<ErrorCode> {
            if let VortexSdkError::Rpc(inner) = self {
                if let Some(TransactionError::InstructionError(_, InstructionError::Custom(code))) =
                    inner.get_transaction_error()
                {
                    // inverse of anchor's 'From<ErrorCode> for u32'
                    return Some(unsafe {
                        std::mem::transmute::<u32, ErrorCode>(
                            code - anchor_lang::error::ERROR_CODE_OFFSET,
                        )
                    });
                }
            }
            None
        }
        /// convert to 'out of sol' error is possible
        pub fn to_out_of_sol_error(&self) -> Option<VortexSdkError> {
            if let VortexSdkError::Rpc(inner) = self {
                if let Some(
                    TransactionError::InsufficientFundsForFee
                    | TransactionError::InsufficientFundsForRent { account_index: _ },
                ) = inner.get_transaction_error()
                {
                    return Some(Self::OutOfSOL);
                }
            }
            None
        }
}
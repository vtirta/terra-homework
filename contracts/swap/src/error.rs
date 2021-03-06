use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("quantity is invalid")]
    InvalidQuantity,

    #[error("Buy Error")]
    BuyError {},

    #[error("Invalid price coming from Oracle")]
    InvalidOraclePriceError {},

    #[error("Invalid amount to transfer")]
    InvalidAmountToTransfer {},

    #[error("Amount must be greater than 0")]
    InvalidWithdrawalAmount,

    #[error("Not enough balance to withdraw")]
    NotEnoughBalanceToWithdraw,

    #[error("Not implemented")]
    NotImplemented {},
}

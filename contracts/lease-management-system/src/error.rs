use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidDenom")]
    InvalidDenom {},

    #[error("NotFound")]
    NotFound {},

    #[error("InvalidRenter")]
    InvalidRenter {},

    #[error("LessThanRent")]
    LessThanRent {},

    #[error("IsNotRented")]
    IsNotRented {},

    #[error("InvalidRentee")]
    InvalidRentee {},

    #[error("IsRented")]
    IsRented {},

    #[error("Allowance is expired")]
    Expired {},
}

use cosmwasm_std::Uint128;
use cw0::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FlatInfo {
    pub renter: String,
    pub rentee: Option<String>,
    pub rent: Uint128,
    pub expires: Option<Expiration>,
}

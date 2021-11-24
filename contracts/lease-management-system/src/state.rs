use cosmwasm_std::{Addr, Coin, Uint128};
use cw0::Expiration;
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FlatInfo {
    pub renter: String,
    pub rentee: Option<String>,
    pub rent: Uint128,
    pub expires: Option<Expiration>,
}

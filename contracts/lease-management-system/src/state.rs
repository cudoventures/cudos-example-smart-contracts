use cosmwasm_std::{Addr, Coin, Uint128};
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw0::Expiration;


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FlatInfo {
    pub renter: String,
    pub rentee: Option<String>,
    pub is_rented: bool,
    pub rent: Uint128,
    pub expires: Option<Expiration>
}

pub const OWNER: Admin = Admin::new("owner");
pub const DENOM: Item<String> = Item::new("denom");
pub const ACCEPTED_COIN: Item<Coin> = Item::new("accepted_coin");
pub const RENTER_TO_FLAT_ID: Map<&Addr, Vec<usize>> = Map::new("renter_to_flat");
pub const FLAT_LIST: Item<Vec<FlatInfo>> = Item::new("flat_list");

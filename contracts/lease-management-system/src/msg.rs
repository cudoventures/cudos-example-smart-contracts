use cosmwasm_std::{StdError, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ExecuteMsg {
    AddProperty { rent: Uint128 },
    AcceptLease { rentee: String, property_id: usize },
    RequestForLease { property_id: usize },
    TerminateLease { property_id: usize },
    PayRent { property_id: usize },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QueryMsg {
    PropertyDetail(usize),
    ShowAllAvailableProperties,
    GetTotalProperties,
}

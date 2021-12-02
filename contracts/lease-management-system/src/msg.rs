use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// it is function to register property
    AddProperty {
        rent: Uint128,
    },
    AcceptLease {
        property_id: Uint128,
    },
    RequestForLease {
        property_id: Uint128,
    },
    TerminateLease {
        property_id: Uint128,
    },
    PayRent {
        property_id: Uint128,
    },
    RejectLease {
        property_id: Uint128,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PropertyDetail { id: Uint128 },
    ShowAllAvailableProperties {},
    GetTotalProperties,
}

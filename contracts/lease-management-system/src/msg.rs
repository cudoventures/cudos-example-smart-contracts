use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ExecuteMsg {
    /// it is function to register property
    AddProperty {
        rent: Uint128,
    },
    AcceptLease {
        property_id: usize,
    },
    RequestForLease {
        property_id: usize,
    },
    TerminateLease {
        property_id: usize,
    },
    PayRent {
        property_id: usize,
    },
    RejectLease {
        property_id: usize,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QueryMsg {
    PropertyDetail(usize),
    ShowAllAvailableProperties,
    GetTotalProperties,
}

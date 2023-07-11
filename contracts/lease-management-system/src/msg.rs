use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ExecuteMsg {
    /// it is function to register property
    RemoveProperty {
        property_id: u16,
    },
    AddProperty {
        rent: Uint128,
    },
    AcceptLease {
        property_id: u16,
    },
    RequestForLease {
        property_id: u16,
    },
    TerminateLease {
        property_id: u16,
    },
    PayRent {
        property_id: u16,
    },
    RejectLease {
        property_id: u16,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QueryMsg {
    PropertyDetail(u16),
    ShowAllAvailableProperties,
    GetTotalProperties,
}

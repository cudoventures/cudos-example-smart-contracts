use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset {
        count: i32,
    },
    CreateGame {
        nought: String,
        zero: String,
    },
    UpdateGame {
        game_id: u64,
        side: bool,
        i: usize,
        j: usize,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},

    GetWinner { game_id: u64 },
    QueryGame { game_id: u64 },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Mode {
    Vertical,
    Horizontal,
    Diagonal,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateGame {
        bet: Coin,
    },
    JoinGame {
        game_id: Uint128,
    },
    WithdrawBet {
        game_id: Uint128,
    },
    UpdateGame {
        game_id: Uint128,
        side: bool,
        i: u16,
        j: u16,
    },
    CancelGame {
        game_id: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetWinner {
        game_id: Uint128,
    },
    QueryGame {
        game_id: Uint128,
    },
    PendingGames {},
    AllGames {
        start_after: Option<Uint128>,
        limit: Option<u32>,
    },
}
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameResult {
    Cross,
    Nought,
    Draw,
    NoResult,
}

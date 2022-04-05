use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::GameBoard;

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
        bet: Coin,
        zero: String,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    GetWinner { game_id: Uint128 },
    QueryGame { game_id: Uint128 },
    // FindWinnerUsingBoard { game: GameBoard },
}
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameResult {
    Nought,
    Zero,
    Draw,
    NoResult,
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub type GameBoard = [[Option<bool>; 3]; 3];
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    pub game: GameBoard,
    pub next_move: bool,
    pub nought: Addr,
    pub zero: Addr,
    pub bet: Coin,
    pub is_pending: bool,
    pub is_completed: bool,
}

impl Game {
    pub fn new(nought: &Addr, zero: &Addr, bet: &Coin) -> Self {
        Game {
            game: [[None, None, None], [None, None, None], [None, None, None]],
            next_move: true,
            nought: nought.clone(),
            zero: zero.clone(),
            bet: bet.clone(),
            is_pending: true,
            is_completed: false,
        }
    }
    pub fn update_game(&mut self, i: usize, j: usize, val: bool) -> bool {
        if !self.is_pending && self.game[i][j] == None {
            self.game[i][j] = Some(val);
            return true;
        }
        false
    }
    pub fn update_side(&mut self) {
        self.next_move = !self.next_move;
    }
    pub fn start_game(&mut self) {
        self.is_pending = false;
    }
    pub fn complete_game(&mut self) {
        self.is_completed = true;
    }
}

pub const STATE: Item<State> = Item::new("state");
pub const GAME_MAP: Map<String, Game> = Map::new("game_map");

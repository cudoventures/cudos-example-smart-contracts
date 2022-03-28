use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    pub game: [[Option<bool>; 3]; 3],
    pub next_move: bool,
    pub nought: Addr,
    pub zero: Addr,
}

impl Game {
    pub fn new(nought: &Addr, zero: &Addr) -> Self {
        Game {
            game: [[None, None, None], [None, None, None], [None, None, None]],
            next_move: true,
            nought: nought.clone(),
            zero: zero.clone(),
        }
    }
    pub fn update_game(&mut self, i: usize, j: usize, val: bool) -> bool {
        if self.game[i][j] == None {
            self.game[i][j] = Some(val);
            return true;
        }
        false
    }
    pub fn update_side(&mut self, next_move: bool) {
        self.next_move = next_move;
    }
}

pub const STATE: Item<State> = Item::new("state");
pub const GAME_MAP: Map<String, Game> = Map::new("game_map");

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct COUNTER {
    pub count: i32,
    pub owner: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameState {
    Pending,
    Started,
    Completed
}

pub type GameBoard = [[Option<bool>; 3]; 3];
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    pub game: GameBoard,
    pub next_move: bool,
    pub cross: Addr,
    pub nought: Option<Addr>,
    pub bet: Coin,
    pub state: GameState,
}

impl Game {
    pub fn new(cross: &Addr, bet: &Coin) -> Self {
        Game {
            game: [[None, None, None], [None, None, None], [None, None, None]],
            next_move: true,
            cross: cross.clone(),
            bet: bet.clone(),
            nought: None,
            state: GameState::Pending,
        }
    }
    pub fn update_opponent(&mut self, nought: &Addr) {
        self.nought = Some(nought.clone());
    }
    pub fn update_game(&mut self, i: u16, j: u16, val: bool) -> bool {
        if self.state == GameState::Started && self.game[i as usize][j as usize] == None {
            self.game[i as usize][j as usize] = Some(val);
            return true;
        }
        false
    }
    pub fn update_side(&mut self) {
        self.next_move = !self.next_move;
    }
    pub fn start_game(&mut self) {
        self.state = GameState::Started;
    }
    pub fn complete_game(&mut self) {
        self.state = GameState::Completed;
    }
}
pub const GAME_MAP: Map<String, Game> = Map::new("game_map");

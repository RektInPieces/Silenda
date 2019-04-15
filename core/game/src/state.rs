use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use crate::rules::Card;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
  // Address -> PlayerState
  pub players: HashMap<u16, PlayerState>,
  pub deck: Vec<Card>
}

impl Default for State {
  fn default() -> Self {
    State {
      players: HashMap::new(),
      deck: vec![]
    }
  }
}

impl State {
  fn draw_card(&mut self, _player: u16) {
    
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
  pub cards: Vec<usize>,
  pub score: u32
}
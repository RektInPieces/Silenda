use std::collections::HashMap;
use self::rules::Card;

#[derive(Serialize, Deseria lize, Clone, Debug)]
pub struct GameState<'a> {
  pub turn: u32,
  // Address -> PlayerState
  pub players: HashMap<u16, PlayerState>
}

impl Default for GameState {
  fn default() -> Self {
    GameState {
      cells: [-1; 9]
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState<'a> {
  pub cards: Vec<&'a Card>,
  pub score: u32
}
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
  // Address -> PlayerState
  pub players: HashMap<u16, PlayerState>,
  pub deck: Vec<usize>
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
  pub fn draw_card(&mut self, player: u16) -> Result<(), &str> {
    let card = self.deck.pop().ok_or("No card exists on the deck")?;
    self.players.get_mut(&player)
      .ok_or("Invalid player id")?
      .cards.push(card);
    Ok(())
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
  pub cards: Vec<usize>,
  pub score: u32
}
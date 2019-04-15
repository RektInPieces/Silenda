use std::collections::HashSet;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

pub type PlayerId = u16;
pub type CardId = usize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
  pub players: HashMap<PlayerId, PlayerState>,
  pub deck: Vec<CardId>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardState {
  pub id: CardId,
  pub visible_to: HashSet<PlayerId>
}
impl CardState {
  pub fn new(id: CardId) -> CardState {
    CardState {
      id: id,
      visible_to: HashSet::new()
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
  pub cards: Vec<CardState>,
  pub score: u32
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
  pub fn draw_card(&mut self, player: PlayerId) -> Result<(), &str> {
    let card_id = self.deck.pop().ok_or("No card exists on the deck")?;
    self.players.get_mut(&player)
      .ok_or("Invalid player id")?
      .cards.push(CardState::new(card_id));
    Ok(())
  }

  pub fn reveal_card(&mut self, player: PlayerId, card: usize, to: &Vec<PlayerId>) -> Result<(), &str> {
    let card_state =
      self.players.get_mut(&player).ok_or("Invalid player id")?
      .cards.get_mut(card).ok_or("Invalid card id")?;

    for player in to {
      card_state.visible_to.insert(*player);
    }
    Ok(())
  }
}

#[cfg(test)]
use std::error::Error;

#[test]
fn drawing_card() -> Result<(), Box<Error>> {
  let mut state = State::default();
  const CARD_ID: usize = 0;
  const PLAYER_ID: u16 = 0;

  state.deck.push(CARD_ID);
  state.players.insert(PLAYER_ID, PlayerState { cards: vec![], score: 0 });
  state.draw_card(PLAYER_ID)?;
  assert_eq!(state.players[&PLAYER_ID].cards.len(), 1);
  assert_eq!(state.players[&PLAYER_ID].cards[0].id, CARD_ID);

  Ok(())
}
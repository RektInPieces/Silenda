use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameRules {
  pub name: String,
  pub cards: Vec<Card>,
  pub hooks: Vec<Hook>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
  pub name: String,
  pub icon: String,
  // ipfs
  pub color: Option<u8>,
  pub suit: Option<u8>,
  pub value: i32,
  pub action: Vec<CardAction>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardAction {
  pub condition: String,
  pub action: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hook {
  pub on: HookTime,
  pub condition: String,
  pub action: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HookTime {
  EveryTurn
}


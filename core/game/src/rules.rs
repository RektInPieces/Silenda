use std::cmp::Ordering;

pub struct GameRules {
  pub name: String,
  pub cards: Vec<Card>,
  pub hooks: Vec<Hook>
}

pub struct Card {
  pub name: String,
  pub icon: String,
  // ipfs
  pub color: Option<u8>,
  pub suit: Option<u8>,
  pub value: i32,
  pub action: Vec<CardAction>
}

pub struct CardAction {
  pub condition: String,
  pub action: String
}

pub struct Hook {
  pub on: HookTime,
  pub condition: String,
  pub action: String
}

pub enum HookTime {
  EveryTurn
}


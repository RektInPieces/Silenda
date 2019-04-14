use std::cmp::Ordering;

pub enum Player {
  CurrentPlayer, TargetedPlayer
}

pub struct GameRules {
  pub name: &str,
  pub cards: Vec<Card>,
  pub hooks: Vec<Hook>
}

pub struct Card {
  pub name: &str,
  pub icon: &str,
  pub color: &str,
  pub suit: &str,
  pub value: i32,
  pub action: CardAction
}

pub struct CardAction {
  pub condition: Condition,
  pub action: Action
}

pub struct Hook {
  pub on: HookTime,
  pub condition: Condition,
  pub action: Action
}

pub enum HookTime {
  EveryTurn
}

pub enum Action {
  ChangeScore { target: Player, amount: i32 },
  EliminatePlayer { target: Player },
  EndGame { winner: Player }
}

pub enum Condition {
  CurrentPlayerScore { op: Ordering, value: i32 },
  TargetDoesNotHaveCard { target: Player, card: Card },
  TargetHasCard { target: Player, card: Card },
  TargetHasColor {target: Player, card: Card},
  TargetHasSuit {target: Player, card: Card},
  TargetCardValue { op: Ordering, value: i32 }
}
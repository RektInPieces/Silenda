pub struct GameRules {
  pub name: &str,
  pub cards: Vec<Card>
  pub hooks: Vec<Hook>
}

pub struct Card {
  pub name: &str,
  pub icon: &str
  pub value: i32
}

pub struct Hook {
  pub on: HookTime,
  pub condition: HookCondition,
  pub action: Action
}

pub enum HookTime {
  EveryTurn
}

pub struct Action {
  
}
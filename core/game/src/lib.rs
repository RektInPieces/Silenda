mod rules;
mod state;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate quick_error;

extern crate oasis_game_core;
#[macro_use]
extern crate oasis_game_core_derive;
extern crate rhai;

use serde_json::Value;
use std::error::Error;
use oasis_game_core::{*, Game as InnerGame};
use oasis_game_core_derive::{flow, moves};
use rhai::Engine;

/// Error types.
quick_error! {
    #[derive(Debug)]
    pub enum Errors {
        InvalidCell {
            description("invalid cell")
            display("A move must specify a valid cell.")
        }
    }
}

/// Define your moves as methods in this trait.
#[moves]
trait Moves {
    fn draw_card(state: &mut UserState<state::GameState>, player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {
        
    }

    fn place_card(state: &mut UserState<state::GameState>, player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {
    
    }

    fn reveal_card(state: &mut UserState<state::GameState>, player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {

    }


    
    fn click_cell(state: &mut UserState<state::GameState>, player_id: u16, args: &Option<Value>)
                -> Result<(), Box<Error>> {
        if let Some(value) = args {
            let id = value.as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|cell| cell.as_u64())
                .and_then(|cell| Some(cell as usize))
                .ok_or(Box::new(Errors::InvalidCell))?;
            match state.g.cells[id] {
                -1 => {
                    state.g.cells[id] = state.ctx.current_player as i32;
                    Ok(())
                },
                _ => Err(Box::new(Errors::InvalidCell))
            }
        } else {
            return Err(Box::new(Errors::InvalidCell))
        }
    }
}

/// Define the game flow.
#[flow]
trait Flow {
    fn initial_state(&self, seed: Option<u128>) -> State {
        Default::default()
    }

    fn end_turn_if(&self, _: &UserState<State>) -> bool {
        // End the turn after every move.
        true
    }

    fn end_game_if(&self, state: &UserState<State>) -> Option<(Option<Score>, Value)> {
        // TODO: Make a macro to simplify returning JSON values.
        // TODO: The error handling case for these flow methods is still inadequate.
        if let Some(pos) = is_victory(state.g.cells) {
            let winner = state.ctx.current_player;
            return Some((Some(Score::Win(winner)), json!({
                "winner": winner,
                "winning_cells": pos
            })));
        }
        if state.g.cells.into_iter().all(|c| *c != -1) {
            return Some((Some(Score::Draw), json!({
                "draw": true
            })));
        }
        None
    }
}
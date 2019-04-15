mod rules;
mod state;

use quick_error::quick_error;
use std::error::Error;
use oasis_game_core::*;
use oasis_game_core_derive::{flow, moves};
use serde_json::value::Value;
use state::State;

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
    fn draw_card(state: &mut UserState<State>, _player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {
        if let Some(value) = args {
            let target = value.as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|cell| cell.as_u64())
                .and_then(|cell| Some(cell as u16))
                .ok_or(Box::new(Errors::InvalidCell))?;
            state.g.draw_card(target)?;
        } else {
            return Err(Box::new(Errors::InvalidCell))
        }
        Ok(())
    }

    // fn place_card(state: &mut UserState<State>, player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {
    
    // }

    // fn reveal_card(state: &mut UserState<State>, player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {

    // }
    
    // fn click_cell(state: &mut UserState<State>, player_id: u16, args: &Option<Value>)
    //             -> Result<(), Box<Error>> {
    //     if let Some(value) = args {
    //         let id = value.as_array()
    //             .and_then(|arr| arr.get(0))
    //             .and_then(|cell| cell.as_u64())
    //             .and_then(|cell| Some(cell as usize))
    //             .ok_or(Box::new(Errors::InvalidCell))?;
    //         match state.g.cells[id] {
    //             -1 => {
    //                 state.g.cells[id] = state.ctx.current_player as i32;
    //                 Ok(())
    //             },
    //             _ => Err(Box::new(Errors::InvalidCell))
    //         }
    //     } else {
    //         return Err(Box::new(Errors::InvalidCell))
    //     }
    // }
}

/// Define the game flow.
#[flow]
trait Flow {
    fn initial_state(&self, _seed: Option<u128>) -> State {
        Default::default()
    }

    // fn end_turn_if(&self, _: &UserState<State>) -> bool {
    //     // End the turn after every move.
    //     true
    // }

    // fn end_game_if(&self, state: &UserState<State>) -> Option<(Option<Score>, Value)> {
    //     // TODO: Make a macro to simplify returning JSON values.
    //     // TODO: The error handling case for these flow methods is still inadequate.
    //     if let Some(pos) = is_victory(state.g.cells) {
    //         let winner = state.ctx.current_player;
    //         return Some((Some(Score::Win(winner)), json!({
    //             "winner": winner,
    //             "winning_cells": pos
    //         })));
    //     }
    //     if state.g.cells.into_iter().all(|c| *c != -1) {
    //         return Some((Some(Score::Draw), json!({
    //             "draw": true
    //         })));
    //     }
    //     None
    // }
}
mod rules;
mod state;

use quick_error::quick_error;
use std::error::Error;
use oasis_game_core::*;
use oasis_game_core_derive::{flow, moves};
use serde_json::value::Value;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use state::State;

quick_error! {
    #[derive(Debug)]
    pub enum Errors {
        InvalidMove {
            description("invalid move specified")
            display("invalid move specified")
        }
    }
}

#[derive(FromPrimitive)]
enum Moves {
    DrawCard = 1,
    RevealCard = 2
}

/// Define your moves as methods in this trait.
#[moves]
trait Moves {
    // Seems like the move macro only allows one move function to be specified.
    // In this case, we simply use the first argument given to the function
    // to disambiguate which method to call.
    // This also can't be called "make_move" because the macro defines its own
    // function called make_move
    fn do_move(state: &mut UserState<State>, player_id: u16, args: &Option<Value>) -> Result<(), Box<Error>> {
        let args = args.as_ref().ok_or(Box::new(Errors::InvalidMove))?;
        let move_id = args.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|arg0| arg0.as_u64())
            .ok_or(Box::new(Errors::InvalidMove))?;
        match FromPrimitive::from_u64(move_id) {
            Some(Moves::DrawCard) => {
                let target = args.as_array()
                    .and_then(|arr| arr.get(1))
                    .and_then(|arg0| arg0.as_u64())
                    .ok_or(Box::new(Errors::InvalidMove))?;
                state.g.draw_card(target as u16)?;
            }
            Some(Moves::RevealCard) => {
                let card_index = args.as_array()
                    .and_then(|arr| arr.get(1))
                    .and_then(|arg0| arg0.as_u64())
                    .ok_or(Box::new(Errors::InvalidMove))? as usize;
                let visible_to = args.as_array()
                    .and_then(|arr| arr.get(2))
                    .and_then(|arg0| arg0.as_array())
                    .ok_or(Box::new(Errors::InvalidMove))?
                    // HACK
                    .iter().map(|val| val.as_u64().unwrap_or(0) as u16).collect();
                state.g.reveal_card(player_id, card_index, &visible_to)?;
            }
            Some(Moves::PlaceCard) => {
                let card_index = args.as_array()
                    .and_then(|arr| arr.get(1))
                    .and_then(|arg0| arg0.as_u64())
                    .ok_or(Box::new(Errors::InvalidMove))?;
                let target = args.as_array() 
                    .and_then(|arr| arr.get(1))
                    .and_then(|arg0| arg0.as_i64())
                    .ok_or(Box::new(Errors::InvalidMove))?;
                    //HACK
                    .iter().map(|val| val.as_u64().unwrap_or(0) as u16).collect();
                state.g.place_card(player_id, card_index, target)
            }
            None => return Err(Box::new(Errors::InvalidMove))
        }
        Ok(())
    }
    
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
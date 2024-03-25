use yewdux::prelude::*;
use crate::entities::board::{Board, Square};


#[derive(PartialEq, Clone, Store)]
pub struct GameState {
    pub pawn_to_move: Option<Square>,
    pub board: Board,
}

impl GameState {
    pub fn select_or_move_pawn(&mut self, square_target: Square) {
        if self.pawn_to_move.is_none() {
            self.board.toggle_pawn_highlight(&square_target);
            self.pawn_to_move = Some(square_target);
        } else {
            match &self.pawn_to_move {
                Some(p) if *p == square_target => {
                    self.board.toggle_pawn_highlight(&square_target);
                    self.pawn_to_move = None;
                },
                Some(_) => {
                    self.board.move_pawn(&square_target,&self.pawn_to_move.unwrap());
                    self.board.toggle_pawn_highlight(&square_target);
                    self.pawn_to_move = None;
                },
                None => unreachable!(),
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pawn_to_move: Default::default(),
            board: Default::default()
        }
    }
}

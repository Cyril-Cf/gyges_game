use crate::{
    constant::Player,
    entities::board::{Board, Square},
};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Store)]
pub struct GameState {
    pub pawn_to_move: Option<Square>,
    pub board: Board,
    pub player_turn: Player,
}

impl GameState {
    pub fn select_or_move_pawn(&mut self, square_target: Square) {
        if self.pawn_to_move.is_none() {
            self.board.toggle_pawn_highlight(&square_target);
            self.pawn_to_move = Some(square_target);
            self.board
                .update_possible_moves(&square_target, &self.player_turn);
        } else {
            self.board.clear_all_squares_can_move_to();
            self.board.toggle_pawn_highlight(&square_target);
            match &self.pawn_to_move {
                Some(p)
                    if p.line_index == square_target.line_index
                        && p.pawn_index == square_target.pawn_index =>
                {
                    self.board.remove_possible_moves();
                    self.board.remove_temp_moves();
                    self.pawn_to_move = None;
                }
                Some(_) => {
                    self.board
                        .move_pawn(&self.pawn_to_move.unwrap(), &square_target);
                    self.switch_player();
                    self.pawn_to_move = None;
                    self.board.toggle_pawn_highlight(&square_target);
                }
                None => unreachable!(),
            }
        }
    }
    fn switch_player(&mut self) {
        if self.player_turn == Player::PlayerOne {
            self.player_turn = Player::PlayerTwo;
        } else {
            self.player_turn = Player::PlayerOne;
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pawn_to_move: Default::default(),
            board: Default::default(),
            player_turn: Player::PlayerTwo,
        }
    }
}

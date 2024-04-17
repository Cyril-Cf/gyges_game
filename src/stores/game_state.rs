use crate::{
    constant::{GameStatus, Player, PreparationStep},
    entities::board::{is_closest_line_to_player, Board, CorrectPath, Pawn, Square},
};
use rand::seq::SliceRandom;
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Store)]
pub struct GameState {
    pub pawn_to_move: Option<Square>,
    pub board: Board,
    pub player_turn: Player,
    pub status: GameStatus,
    pub winning_player: Option<Player>,
}

impl GameState {
    pub fn select_or_move_pawn(
        &mut self,
        square_target: Option<Square>,
        winning_player: Option<Player>,
    ) {
        match winning_player {
            Some(Player::PlayerTop) => {
                let board = self.board.clone();
                let last_line = board.lines.last().unwrap();
                if let Some(square) = last_line.squares.first() {
                    self.board.move_pawn(&self.pawn_to_move.unwrap(), square);
                    self.status = GameStatus::Finished;
                    self.winning_player = winning_player;
                    self.remove_all_correct_paths();
                    return;
                }
            }
            Some(Player::PlayerBottom) => {
                let board = self.board.clone();
                let first_line = board.lines.first().unwrap();
                if let Some(square) = first_line.squares.first() {
                    self.board.move_pawn(&self.pawn_to_move.unwrap(), square);
                    self.status = GameStatus::Finished;
                    self.winning_player = winning_player;
                    self.remove_all_correct_paths();
                    return;
                }
            }
            _ => {}
        }
        if let Some(square_target) = square_target {
            if self.pawn_to_move.is_none() {
                if !is_closest_line_to_player(&mut self.board, &self.player_turn, &square_target) {
                    return;
                }
                self.board.toggle_pawn_highlight(&square_target);
                self.pawn_to_move = Some(square_target);
                self.board
                    .update_possible_moves(&square_target, &self.player_turn);
            } else {
                match &self.pawn_to_move {
                    Some(p)
                        if p.line_index == square_target.line_index
                            && p.pawn_index == square_target.pawn_index =>
                    {
                        self.board.clear_all_squares_can_move_to();
                        self.board.toggle_pawn_highlight(&square_target);
                        self.board.remove_possible_moves();
                        self.board.remove_temp_moves();
                        self.pawn_to_move = None;
                    }
                    Some(_) => {
                        if let Some(_) = square_target.pawn {
                            return;
                        }
                        self.board.clear_all_squares_can_move_to();
                        self.board.toggle_pawn_highlight(&square_target);
                        self.board
                            .move_pawn(&self.pawn_to_move.unwrap(), &square_target);
                        self.switch_player();
                        self.remove_all_correct_paths();
                        self.pawn_to_move = None;
                        self.board.toggle_pawn_highlight(&square_target);
                    }
                    None => unreachable!(),
                }
            }
        }
    }
    fn switch_player(&mut self) {
        if self.player_turn == Player::PlayerTop {
            self.player_turn = Player::PlayerBottom;
        } else {
            self.player_turn = Player::PlayerTop;
        }
    }
    pub fn restart_game(&mut self) {
        *self = GameState::default();
    }
    pub fn place_pawns_full_random(&mut self) {
        self.place_pawns_random_player_top();
        self.place_pawns_random_player_bottom();
        self.status = GameStatus::Preparing(PreparationStep::BothPlayersReady);
    }
    pub fn place_pawns_random_player_top(&mut self) {
        let mut pawns = Pawn::get_set_of_pawns();
        pawns.shuffle(&mut rand::thread_rng());
        for square in self.board.lines.get_mut(1).unwrap().squares.iter_mut() {
            square.pawn = Some(pawns.pop().unwrap());
        }
        if self.status == GameStatus::Preparing(PreparationStep::PlayerBottomReady) {
            self.status = GameStatus::Preparing(PreparationStep::BothPlayersReady);
        } else {
            self.status = GameStatus::Preparing(PreparationStep::PlayerTopReady);
        }
    }
    pub fn place_pawns_random_player_bottom(&mut self) {
        let mut pawns = Pawn::get_set_of_pawns();
        pawns.shuffle(&mut rand::thread_rng());
        for square in self.board.lines.get_mut(6).unwrap().squares.iter_mut() {
            square.pawn = Some(pawns.pop().unwrap());
        }
        if self.status == GameStatus::Preparing(PreparationStep::PlayerTopReady) {
            self.status = GameStatus::Preparing(PreparationStep::BothPlayersReady);
        } else {
            self.status = GameStatus::Preparing(PreparationStep::PlayerBottomReady);
        }
    }
    pub fn start_game(&mut self) {
        self.status = GameStatus::Playing;
    }
    pub fn chose_player_top(&mut self) {
        self.player_turn = Player::PlayerTop;
    }
    pub fn chose_player_bottom(&mut self) {
        self.player_turn = Player::PlayerBottom;
    }
    pub fn trace_correct_path(&mut self, square_target: &Square) {
        let correct_paths: Vec<&CorrectPath> = self
            .board
            .possible_moves
            .iter()
            .filter(|path| {
                let last_move = path.moves.last().unwrap();
                square_target.line_index == last_move.line_index_to
                    && square_target.pawn_index == last_move.square_index_to
            })
            .collect();
        for path in correct_paths.iter() {
            for single_move in path.moves.iter() {
                self.board
                    .lines
                    .get_mut(single_move.line_index_to)
                    .unwrap()
                    .squares
                    .get_mut(single_move.square_index_to)
                    .unwrap()
                    .is_correct_path = true;
            }
        }
    }

    pub fn remove_all_correct_paths(&mut self) {
        self.board
            .lines
            .iter_mut()
            .for_each(|l| l.squares.iter_mut().for_each(|s| s.is_correct_path = false));
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pawn_to_move: Default::default(),
            board: Default::default(),
            player_turn: Player::PlayerBottom,
            status: GameStatus::Preparing(PreparationStep::NoPlayerReady),
            winning_player: None,
        }
    }
}

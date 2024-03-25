use crate::constant::{PawnType, Player};
use yewdux::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pawn {
    pub player: Player,
    pub paywn_type: PawnType,
    pub position: usize,
    pub is_highlighted: bool,
}

#[derive(Clone, PartialEq, Store)]
pub struct GameState {
    pub pawn_to_move: Option<usize>,
    pub position_after: Option<usize>,
    pub board: [Option<Pawn>; 36],
}

impl GameState {
    pub fn select_or_move_pawn(&mut self, index: usize) {
        if self.pawn_to_move.is_none() {
            self.toogle_pawn_highlight(index);
            self.pawn_to_move = Some(index);
        } else if index == self.pawn_to_move.unwrap() {
            self.toogle_pawn_highlight(index);
            self.pawn_to_move = None;
        } else {
            self.move_pawn(index, self.pawn_to_move.unwrap());
            self.toogle_pawn_highlight(index);
            self.pawn_to_move = None;
        }
    }
    fn move_pawn(&mut self, pawn_to_move: usize, position_after: usize) {
        self.board.swap(pawn_to_move, position_after);
    }

    fn toogle_pawn_highlight(&mut self, index: usize) {
        self.board[index].as_mut().unwrap().is_highlighted = !self.board.get(index).unwrap().unwrap().is_highlighted ;
    }

}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pawn_to_move: Default::default(),
            position_after: Default::default(),
            board: [
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::One,
                    position: 0,
                    is_highlighted: false
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::One,
                    position: 1,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Two,
                    position: 2,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Two,
                    position: 3,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Three,
                    position: 4,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Three,
                    position: 05,
                    is_highlighted: Default::default()
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::One,
                    position: 30,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::One,
                    position: 31,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Two,
                    position: 32,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Two,
                    position: 33,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Three,
                    position: 34,
                    is_highlighted: Default::default()
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Three,
                    position: 35,
                    is_highlighted: Default::default()
                }),
            ],
        }
    }
}

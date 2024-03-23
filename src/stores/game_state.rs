use crate::constant::{PawnType, Player};
use crate::state::Pawn;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store)]
pub struct GameState {
    pub count: u32,
    pub board: [Option<Pawn>; 36],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            count: Default::default(),
            board: [
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::One,
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::One,
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Two,
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Two,
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Three,
                }),
                Some(Pawn {
                    player: Player::PlayerOne,
                    paywn_type: PawnType::Three,
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
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::One,
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Two,
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Two,
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Three,
                }),
                Some(Pawn {
                    player: Player::PlayerTwo,
                    paywn_type: PawnType::Three,
                }),
            ],
        }
    }
}

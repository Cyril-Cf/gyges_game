use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::constant::{PawnType, GameStatus, Player};
// use crate::helper::shuffle_cards;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pawn {
    pub player: Player,
    pub paywn_type: PawnType,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub status: GameStatus,
    pub board: [Option<Pawn>; 36],
}


pub enum Action {
    GameReset,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::GameReset => State::reset().into(),
        }
    }
}

impl State {
    pub fn reset() -> State {
        State {
            // board: [None; 36],
            board: [
                Some(Pawn{player: Player::PlayerOne, paywn_type: PawnType::One}),
                Some(Pawn{player: Player::PlayerOne, paywn_type: PawnType::One}),
                Some(Pawn{player: Player::PlayerOne, paywn_type: PawnType::Two}),
                Some(Pawn{player: Player::PlayerOne, paywn_type: PawnType::Two}),
                Some(Pawn{player: Player::PlayerOne, paywn_type: PawnType::Three}),
                Some(Pawn{player: Player::PlayerOne, paywn_type: PawnType::Three}),
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
                Some(Pawn{player: Player::PlayerTwo, paywn_type: PawnType::One}),
                Some(Pawn{player: Player::PlayerTwo, paywn_type: PawnType::One}),
                Some(Pawn{player: Player::PlayerTwo, paywn_type: PawnType::Two}),
                Some(Pawn{player: Player::PlayerTwo, paywn_type: PawnType::Two}),
                Some(Pawn{player: Player::PlayerTwo, paywn_type: PawnType::Three}),
                Some(Pawn{player: Player::PlayerTwo, paywn_type: PawnType::Three}),
            ],
            status: GameStatus::Ready,
        }
    }
}

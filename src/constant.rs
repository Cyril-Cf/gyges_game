use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PawnType {
    One,
    Two,
    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    Ready,
    Playing,
    Finished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckMove {
    Top,
    Bottom,
    Left,
    Right,
}

impl CheckMove {
    pub fn into_iter() -> core::array::IntoIter<CheckMove, 4> {
        [
            CheckMove::Top,
            CheckMove::Bottom,
            CheckMove::Left,
            CheckMove::Right,
        ]
        .into_iter()
    }
}

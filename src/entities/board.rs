use crate::constant::{PawnType, Player};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Board {
    pub lines: [Line; 8]
}

impl Default for Board {
    fn default() -> Self {
        Self {
            lines: [
                Line {
                    is_hidden: true,
                    belongs_to: Some(Player::PlayerOne),
                    squares: [Square{ pawn: None, line_index: 0, pawn_index: 0}; 6]
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::One, is_highlighted: false, player: Player::PlayerOne }), line_index: 1, pawn_index: 0},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::One, is_highlighted: false, player: Player::PlayerOne }), line_index: 1, pawn_index: 1},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Two, is_highlighted: false, player: Player::PlayerOne }), line_index: 1, pawn_index: 2},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Two, is_highlighted: false, player: Player::PlayerOne }), line_index: 1, pawn_index: 3},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Three, is_highlighted: false, player: Player::PlayerOne }), line_index: 1, pawn_index: 4},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Three, is_highlighted: false, player: Player::PlayerOne }), line_index: 1, pawn_index: 5},
                    ]
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [Square{ pawn: None, line_index: 2, pawn_index: 0},
                        Square{ pawn: None, line_index: 2, pawn_index: 1},
                        Square{ pawn: None, line_index: 2, pawn_index: 2},
                        Square{ pawn: None, line_index: 2, pawn_index: 3},
                        Square{ pawn: None, line_index: 2, pawn_index: 4},
                        Square{ pawn: None, line_index: 2, pawn_index: 5},
                    ]
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [Square{ pawn: None, line_index: 3, pawn_index: 0},
                        Square{ pawn: None, line_index: 3, pawn_index: 1},
                        Square{ pawn: None, line_index: 3, pawn_index: 2},
                        Square{ pawn: None, line_index: 3, pawn_index: 3},
                        Square{ pawn: None, line_index: 3, pawn_index: 4},
                        Square{ pawn: None, line_index: 3, pawn_index: 5},
                    ]
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [Square{ pawn: None, line_index: 4, pawn_index: 0},
                        Square{ pawn: None, line_index: 4, pawn_index: 1},
                        Square{ pawn: None, line_index: 4, pawn_index: 2},
                        Square{ pawn: None, line_index: 4, pawn_index: 3},
                        Square{ pawn: None, line_index: 4, pawn_index: 4},
                        Square{ pawn: None, line_index: 4, pawn_index: 5},
                    ]
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [Square{ pawn: None, line_index: 5, pawn_index: 0},
                        Square{ pawn: None, line_index: 5, pawn_index: 1},
                        Square{ pawn: None, line_index: 5, pawn_index: 2},
                        Square{ pawn: None, line_index: 5, pawn_index: 3},
                        Square{ pawn: None, line_index: 5, pawn_index: 4},
                        Square{ pawn: None, line_index: 5, pawn_index: 5},
                    ]
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::One, is_highlighted: false, player: Player::PlayerTwo }), line_index: 6, pawn_index: 0},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::One, is_highlighted: false, player: Player::PlayerTwo }), line_index: 6, pawn_index: 1},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Two, is_highlighted: false, player: Player::PlayerTwo }), line_index: 6, pawn_index: 2},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Two, is_highlighted: false, player: Player::PlayerTwo }), line_index: 6, pawn_index: 3},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Three, is_highlighted: false, player: Player::PlayerTwo }), line_index: 6, pawn_index: 4},
                        Square{ pawn: Some(Pawn { paywn_type: PawnType::Three, is_highlighted: false, player: Player::PlayerTwo }), line_index: 6, pawn_index: 5},
                    ]
                },
                Line {
                    is_hidden: true,
                    belongs_to: Some(Player::PlayerTwo),
                    squares: [Square{ pawn: None, line_index: 7, pawn_index: 0}; 6]
                },
            ]
        }
    }
}

impl Board {
    pub fn move_pawn(&mut self, square_from: &Square, square_target: &Square) {
        let mut pawn_1: Option<Pawn> = None;
        let mut pawn_2: Option<Pawn> = None;
        if let Some(line) = self.lines.get(square_from.line_index) {
            if let Some(square_1) = line.squares.get(square_from.pawn_index) {
                pawn_1 = square_1.pawn;
            }
        }
        if let Some(line) = self.lines.get_mut(square_target.line_index) {
            if let Some(square_2) = line.squares.get_mut(square_target.pawn_index) {
                pawn_2 = square_2.pawn;
                square_2.pawn = pawn_1;
            }
        }
        if let Some(line) = self.lines.get_mut(square_from.line_index) {
            if let Some(square_1) = line.squares.get_mut(square_from.pawn_index) {
                square_1.pawn = pawn_2;
            }
        }
    }

    pub fn toggle_pawn_highlight(&mut self, square: &Square) {
        if let Some(line) = self.lines.get_mut(square.line_index) {
            if let Some(square) = line.squares.get_mut(square.pawn_index) {
                if let Some(pawn) = square.pawn.as_mut() {
                    pawn.is_highlighted = !pawn.is_highlighted;
                }
            }
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Line {
    pub is_hidden: bool,
    pub belongs_to: Option<Player>,
    pub squares : [Square; 6]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub pawn: Option<Pawn>,
    pub line_index: usize,
    pub pawn_index: usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pawn {
    pub player: Player,
    pub paywn_type: PawnType,
    pub is_highlighted: bool,
}



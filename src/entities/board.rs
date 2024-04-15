use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

use crate::constant::{CheckMove, PawnType, Player};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub lines: [Line; 8],
    pub possible_moves: Vec<CorrectPath>,
    pub tmp_moves: Vec<TempPath>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            lines: [
                Line {
                    is_hidden: true,
                    belongs_to: Some(Player::PlayerOne),
                    squares: [Square {
                        pawn: None,
                        line_index: 0,
                        pawn_index: 0,
                        is_can_move_to: false,
                    }; 6],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::One,
                                is_highlighted: false,
                                player: Player::PlayerOne,
                            }),
                            line_index: 1,
                            pawn_index: 0,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::One,
                                is_highlighted: false,
                                player: Player::PlayerOne,
                            }),
                            line_index: 1,
                            pawn_index: 1,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Two,
                                is_highlighted: false,
                                player: Player::PlayerOne,
                            }),
                            line_index: 1,
                            pawn_index: 2,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Two,
                                is_highlighted: false,
                                player: Player::PlayerOne,
                            }),
                            line_index: 1,
                            pawn_index: 3,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Three,
                                is_highlighted: false,
                                player: Player::PlayerOne,
                            }),
                            line_index: 1,
                            pawn_index: 4,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Three,
                                is_highlighted: false,
                                player: Player::PlayerOne,
                            }),
                            line_index: 1,
                            pawn_index: 5,
                            is_can_move_to: false,
                        },
                    ],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 0,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 1,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 2,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 3,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 4,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 5,
                            is_can_move_to: false,
                        },
                    ],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 0,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 1,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 2,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 3,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 4,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 5,
                            is_can_move_to: false,
                        },
                    ],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 0,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 1,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 2,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 3,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 4,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 5,
                            is_can_move_to: false,
                        },
                    ],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 0,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 1,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 2,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 3,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 4,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 5,
                            is_can_move_to: false,
                        },
                    ],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::One,
                                is_highlighted: false,
                                player: Player::PlayerTwo,
                            }),
                            line_index: 6,
                            pawn_index: 0,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::One,
                                is_highlighted: false,
                                player: Player::PlayerTwo,
                            }),
                            line_index: 6,
                            pawn_index: 1,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Two,
                                is_highlighted: false,
                                player: Player::PlayerTwo,
                            }),
                            line_index: 6,
                            pawn_index: 2,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Two,
                                is_highlighted: false,
                                player: Player::PlayerTwo,
                            }),
                            line_index: 6,
                            pawn_index: 3,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Three,
                                is_highlighted: false,
                                player: Player::PlayerTwo,
                            }),
                            line_index: 6,
                            pawn_index: 4,
                            is_can_move_to: false,
                        },
                        Square {
                            pawn: Some(Pawn {
                                paywn_type: PawnType::Three,
                                is_highlighted: false,
                                player: Player::PlayerTwo,
                            }),
                            line_index: 6,
                            pawn_index: 5,
                            is_can_move_to: false,
                        },
                    ],
                },
                Line {
                    is_hidden: true,
                    belongs_to: Some(Player::PlayerTwo),
                    squares: [Square {
                        pawn: None,
                        line_index: 7,
                        pawn_index: 0,
                        is_can_move_to: false,
                    }; 6],
                },
            ],
            possible_moves: Vec::new(),
            tmp_moves: Vec::new(),
        }
    }
}

impl Board {
    pub fn move_pawn(&mut self, square_from: &Square, square_target: &Square) {
        // TODO ici plutôt que swap, on va devoir verifier que c'est un square possible soit le dernier d'un CorrectPath.moves

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

        self.remove_possible_moves();
        self.remove_temp_moves();
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

    pub fn update_possible_moves(&mut self, square: &Square, player: &Player) {
        if !is_closest_line_to_player(self, player, square) {
            self.remove_possible_moves();
            return;
        }
        let starting_length = match square.pawn.unwrap().paywn_type {
            PawnType::One => 1 as usize,
            PawnType::Two => 2 as usize,
            PawnType::Three => 3 as usize,
        };
        check_for_moves(self, square.line_index, square.pawn_index, starting_length);
    }

    pub fn remove_possible_moves(&mut self) {
        self.possible_moves = Vec::new();
    }

    pub fn remove_temp_moves(&mut self) {
        self.tmp_moves = Vec::new();
    }

    pub fn clear_all_squares_can_move_to(&mut self) {
        self.lines
            .iter_mut()
            .for_each(|l| l.squares.iter_mut().for_each(|s| s.is_can_move_to = false));
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Line {
    pub is_hidden: bool,
    pub belongs_to: Option<Player>,
    pub squares: [Square; 6],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub pawn: Option<Pawn>,
    pub line_index: usize,
    pub pawn_index: usize,
    pub is_can_move_to: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pawn {
    pub player: Player,
    pub paywn_type: PawnType,
    pub is_highlighted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CorrectPath {
    moves: Vec<Move>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TempPath {
    moves: Vec<Move>,
    remaining_length: usize,
}

impl Into<CorrectPath> for TempPath {
    fn into(self) -> CorrectPath {
        CorrectPath { moves: self.moves }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Move {
    pub line_index_from: usize,
    pub line_index_to: usize,
    pub square_index_from: usize,
    pub square_index_to: usize,
}

fn is_closest_line_to_player(board: &mut Board, player: &Player, square: &Square) -> bool {
    match player {
        Player::PlayerOne => {
            for (index, line) in board.lines.iter().enumerate().skip(1).take(6) {
                if line.squares.iter().any(|s| s.pawn.is_some()) {
                    if index == square.line_index {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        Player::PlayerTwo => {
            for (index, line) in board.lines.iter().rev().enumerate().skip(1).take(6) {
                if line.squares.iter().any(|s| s.pawn.is_some()) {
                    if 7 - index == square.line_index {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
    }
    false
}

fn activate_square_can_move_to(board: &mut Board) {
    board.possible_moves.iter().for_each(|m| {
        let line_index = m.moves.last().unwrap().line_index_to;
        let square_index = m.moves.last().unwrap().square_index_to;
        board
            .lines
            .get_mut(line_index)
            .unwrap()
            .squares
            .get_mut(square_index)
            .unwrap()
            .is_can_move_to = true;
    })
}

fn check_for_moves(
    board: &mut Board,
    starting_line_index: usize,
    starting_square_index: usize,
    starting_length: usize,
) {
    for direction in CheckMove::into_iter() {
        if let Some(single_move) = check_unit_move(
            board,
            starting_line_index,
            starting_square_index,
            starting_length,
            direction,
        ) {
            board.tmp_moves.push(TempPath {
                remaining_length: starting_length - 1,
                moves: vec![single_move],
            })
        }
    }

    while !board.tmp_moves.is_empty() {
        let last_path = board.tmp_moves.pop().unwrap();
        let all_moves = last_path.moves.clone();
        let last_move = last_path.moves.last().unwrap();

        // si remaining == 0 et que le square d'arrivée et vide alors correct
        if last_path.remaining_length == 0 {
            if check_if_square_empty(board, last_move.line_index_to, last_move.square_index_to) {
                if check_if_all_moves_are_correct(&all_moves) {
                    board.possible_moves.push(CorrectPath { moves: all_moves });
                }
            } else {
                gloo::console::log!("last move");
                gloo::console::log!(serde_json::to_string(last_move).unwrap());
                let new_remaining_length = match board
                    .lines
                    .get(last_move.line_index_to)
                    .unwrap()
                    .squares
                    .get(last_move.square_index_to)
                    .unwrap()
                    .pawn
                    .unwrap()
                    .paywn_type
                {
                    PawnType::One => 1 as usize,
                    PawnType::Two => 2 as usize,
                    PawnType::Three => 3 as usize,
                };
                for direction in CheckMove::into_iter() {
                    if let Some(single_move) = check_unit_move(
                        board,
                        last_move.line_index_to,
                        last_move.square_index_to,
                        new_remaining_length,
                        direction,
                    ) {
                        let mut all_moves = last_path.moves.clone();
                        all_moves.push(single_move);
                        if check_if_all_moves_are_correct(&all_moves) {
                            board.tmp_moves.push(TempPath {
                                remaining_length: new_remaining_length - 1,
                                moves: all_moves,
                            });
                        }
                    }
                }
            }
        } else {
            for direction in CheckMove::into_iter() {
                if let Some(single_move) = check_unit_move(
                    board,
                    last_move.line_index_to,
                    last_move.square_index_to,
                    last_path.remaining_length,
                    direction,
                ) {
                    let mut all_moves = last_path.moves.clone();
                    all_moves.push(single_move);
                    board.tmp_moves.push(TempPath {
                        remaining_length: last_path.remaining_length - 1,
                        moves: all_moves,
                    })
                }
            }
        }
    }
    activate_square_can_move_to(board);
}

fn check_if_all_moves_are_correct(moves: &Vec<Move>) -> bool {
    let mut line_moves: HashMap<usize, usize> = HashMap::new();
    let mut row_moves: HashMap<usize, usize> = HashMap::new();
    for single_move in moves {
        if single_move.line_index_from != single_move.line_index_to {
            line_moves.insert(single_move.line_index_from, single_move.line_index_to);
        } else {
            row_moves.insert(single_move.square_index_from, single_move.square_index_to);
        }
    }
    for (&key, &value) in line_moves.iter() {
        if let Some(&other_value) = line_moves.get(&value) {
            if other_value == key {
                return false;
            }
        }
    }
    for (&key, &value) in row_moves.iter() {
        if let Some(&other_value) = row_moves.get(&value) {
            if other_value == key {
                return false;
            }
        }
    }
    true
}

fn check_if_square_empty(board: &Board, line_index: usize, square_index: usize) -> bool {
    if board
        .lines
        .get(line_index)
        .unwrap()
        .squares
        .get(square_index)
        .unwrap()
        .pawn
        .is_none()
    {
        true
    } else {
        false
    }
}

fn check_unit_move(
    board: &Board,
    line_index: usize,
    square_index: usize,
    remaining_length: usize,
    direction: CheckMove,
) -> Option<Move> {
    // check for impossible moves first
    let line_index_to = match direction {
        CheckMove::Top => {
            if line_index > 0 {
                line_index - 1
            } else {
                0
            }
        }
        CheckMove::Bottom => line_index + 1,
        _ => line_index,
    };
    let square_index_to = match direction {
        CheckMove::Left => {
            if square_index > 0 {
                square_index - 1
            } else {
                0
            }
        }
        CheckMove::Right => square_index + 1,
        _ => square_index,
    };

    match direction {
        CheckMove::Top => {
            if line_index == 1 {
                return None;
            }
        }
        CheckMove::Bottom => {
            if line_index == 6 {
                return None;
            }
        }
        CheckMove::Left => {
            if square_index == 0 {
                return None;
            }
        }
        CheckMove::Right => {
            if square_index == 5 {
                return None;
            }
        }
    }
    // if the square is empty
    if check_if_square_empty(board, line_index_to, square_index_to) {
        return Some(Move {
            line_index_from: line_index,
            line_index_to,
            square_index_from: square_index,
            square_index_to,
        });
    // If there is a pawn on the square
    } else {
        // If it's the last stop of a path, then treat that as a correct bounce
        if remaining_length == 1 {
            return Some(Move {
                line_index_from: line_index,
                line_index_to,
                square_index_from: square_index,
                square_index_to,
            });
        } else {
            return None;
        }
    }
}

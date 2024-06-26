use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
                    belongs_to: Some(Player::PlayerTop),
                    squares: [Square {
                        pawn: None,
                        line_index: 0,
                        pawn_index: 0,
                        is_can_move_to: false,
                        is_correct_path: false,
                    }; 6],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: None,
                            line_index: 1,
                            pawn_index: 0,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 1,
                            pawn_index: 1,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 1,
                            pawn_index: 2,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 1,
                            pawn_index: 3,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 1,
                            pawn_index: 4,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 1,
                            pawn_index: 5,
                            is_can_move_to: false,
                            is_correct_path: false,
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
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 1,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 2,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 3,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 4,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 2,
                            pawn_index: 5,
                            is_can_move_to: false,
                            is_correct_path: false,
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
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 1,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 2,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 3,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 4,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 3,
                            pawn_index: 5,
                            is_can_move_to: false,
                            is_correct_path: false,
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
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 1,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 2,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 3,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 4,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 4,
                            pawn_index: 5,
                            is_can_move_to: false,
                            is_correct_path: false,
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
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 1,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 2,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 3,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 4,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 5,
                            pawn_index: 5,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                    ],
                },
                Line {
                    is_hidden: false,
                    belongs_to: None,
                    squares: [
                        Square {
                            pawn: None,
                            line_index: 6,
                            pawn_index: 0,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 6,
                            pawn_index: 1,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 6,
                            pawn_index: 2,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 6,
                            pawn_index: 3,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 6,
                            pawn_index: 4,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                        Square {
                            pawn: None,
                            line_index: 6,
                            pawn_index: 5,
                            is_can_move_to: false,
                            is_correct_path: false,
                        },
                    ],
                },
                Line {
                    is_hidden: true,
                    belongs_to: Some(Player::PlayerBottom),
                    squares: [Square {
                        pawn: None,
                        line_index: 7,
                        pawn_index: 0,
                        is_can_move_to: false,
                        is_correct_path: false,
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
        if square_target.is_can_move_to {
            let mut pawn_1: Option<Pawn> = None;
            if let Some(line) = self.lines.get(square_from.line_index) {
                if let Some(square_1) = line.squares.get(square_from.pawn_index) {
                    pawn_1 = square_1.pawn;
                }
            }
            if let Some(line) = self.lines.get_mut(square_target.line_index) {
                if let Some(square_2) = line.squares.get_mut(square_target.pawn_index) {
                    square_2.pawn = pawn_1;
                }
            }
            if let Some(line) = self.lines.get_mut(square_from.line_index) {
                if let Some(square_1) = line.squares.get_mut(square_from.pawn_index) {
                    square_1.pawn = None;
                }
            }
            self.remove_possible_moves();
            self.remove_temp_moves();
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
        check_for_moves(
            self,
            square.line_index,
            square.pawn_index,
            starting_length,
            player,
        );
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

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct Square {
    pub pawn: Option<Pawn>,
    pub line_index: usize,
    pub pawn_index: usize,
    pub is_can_move_to: bool,
    pub is_correct_path: bool,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct Pawn {
    pub paywn_type: PawnType,
    pub is_highlighted: bool,
}

impl Pawn {
    pub fn get_set_of_pawns() -> Vec<Pawn> {
        vec![
            Pawn {
                is_highlighted: false,
                paywn_type: PawnType::One,
            },
            Pawn {
                is_highlighted: false,
                paywn_type: PawnType::One,
            },
            Pawn {
                is_highlighted: false,
                paywn_type: PawnType::Two,
            },
            Pawn {
                is_highlighted: false,
                paywn_type: PawnType::Two,
            },
            Pawn {
                is_highlighted: false,
                paywn_type: PawnType::Three,
            },
            Pawn {
                is_highlighted: false,
                paywn_type: PawnType::Three,
            },
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CorrectPath {
    pub moves: Vec<Move>,
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Move {
    pub line_index_from: usize,
    pub line_index_to: usize,
    pub square_index_from: usize,
    pub square_index_to: usize,
}

pub fn is_closest_line_to_player(board: &mut Board, player: &Player, square: &Square) -> bool {
    match player {
        Player::PlayerTop => {
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
        Player::PlayerBottom => {
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
    player: &Player,
) {
    for direction in CheckMove::into_iter() {
        if let Some(single_move) = check_unit_move(
            board,
            starting_line_index,
            starting_square_index,
            starting_length,
            direction,
            player,
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

        if last_path.remaining_length == 0 {
            if check_if_square_empty(board, last_move.line_index_to, last_move.square_index_to) {
                if check_if_all_moves_are_correct(&all_moves) {
                    board.possible_moves.push(CorrectPath { moves: all_moves });
                }
            } else {
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
                        player,
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
                    player,
                ) {
                    let mut all_moves = last_path.moves.clone();
                    all_moves.push(single_move);
                    if check_if_all_moves_are_correct(&all_moves) {
                        board.tmp_moves.push(TempPath {
                            remaining_length: last_path.remaining_length - 1,
                            moves: all_moves,
                        })
                    }
                }
            }
        }
    }
    activate_square_can_move_to(board);
    check_for_winning_moves(board);
}

fn check_for_winning_moves(board: &mut Board) {
    if board
        .lines
        .first()
        .unwrap()
        .squares
        .iter()
        .any(|s| s.is_can_move_to)
    {
        board
            .lines
            .first_mut()
            .unwrap()
            .squares
            .iter_mut()
            .for_each(|s| s.is_can_move_to = true);
    };
    if board
        .lines
        .last()
        .unwrap()
        .squares
        .iter()
        .any(|s| s.is_can_move_to)
    {
        board
            .lines
            .last_mut()
            .unwrap()
            .squares
            .iter_mut()
            .for_each(|s| s.is_can_move_to = true);
    };
}

fn check_if_all_moves_are_correct(moves: &Vec<Move>) -> bool {
    let mut seen_moves = HashSet::new();
    for single_move in moves {
        let opposite_move = Move {
            line_index_from: single_move.line_index_to,
            line_index_to: single_move.line_index_from,
            square_index_from: single_move.square_index_to,
            square_index_to: single_move.square_index_from,
        };
        if seen_moves.contains(&opposite_move) || seen_moves.contains(single_move) {
            return false;
        }
        seen_moves.insert(single_move);
    }
    true
}

fn check_if_square_empty(board: &Board, line_index: usize, square_index: usize) -> bool {
    board
        .lines
        .get(line_index)
        .unwrap()
        .squares
        .get(square_index)
        .unwrap()
        .pawn
        .is_none()
}

fn check_unit_move(
    board: &Board,
    line_index: usize,
    square_index: usize,
    remaining_length: usize,
    direction: CheckMove,
    player: &Player,
) -> Option<Move> {
    // check for impossible moves first
    let line_index_to = match direction {
        CheckMove::Top => {
            if line_index > 0 {
                line_index - 1
            } else {
                return None;
            }
        }
        CheckMove::Bottom => {
            if line_index < 7 {
                line_index + 1
            } else {
                return None;
            }
        }
        _ => line_index,
    };
    let square_index_to = match direction {
        CheckMove::Left => {
            if square_index > 0 {
                square_index - 1
            } else {
                return None;
            }
        }
        CheckMove::Right => {
            if square_index < 5 {
                square_index + 1
            } else {
                return None;
            }
        }
        _ => square_index,
    };

    match direction {
        CheckMove::Top => {
            if *player == Player::PlayerTop && line_index == 1 {
                return None;
            }
        }
        CheckMove::Bottom => {
            if *player == Player::PlayerBottom && line_index == 6 {
                return None;
            }
        }
        _ => {}
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

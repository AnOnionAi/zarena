#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "wasm")]
pub mod wasm;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
//
// Constants
//
pub const EMPTY_SQUARE_ID: isize = 0;
pub const KING_ID: isize = 1;
pub const QUEEN_ID: isize = 2;
pub const ROOK_ID: isize = 3;
pub const BISHOP_ID: isize = 4;
pub const KNIGHT_ID: isize = 5;
pub const PAWN_ID: isize = 6;

// const WIN_REWARD: isize = 1;
// const LOSS_REWARD: isize = -1;

const KING_DESC: &str = &"K";
const QUEEN_DESC: &str = &"Q";
const ROOK_DESC: &str = &"R";
const BISHOP_DESC: &str = &"B";
const KNIGHT_DESC: &str = &"N";
const PAWN_DESC: &str = &" ";

#[allow(dead_code)]
const CASTLE_KING_SIDE_WHITE: &str = "CASTLE_KING_SIDE_WHITE";
#[allow(dead_code)]
const CASTLE_QUEEN_SIDE_WHITE: &str = "CASTLE_QUEEN_SIDE_WHITE";
#[allow(dead_code)]
const CASTLE_KING_SIDE_BLACK: &str = "CASTLE_KING_SIDE_BLACK";
#[allow(dead_code)]
const CASTLE_QUEEN_SIDE_BLACK: &str = "CASTLE_QUEEN_SIDE_BLACK";

#[allow(dead_code)]
pub const DEFAULT_BOARD: Board = [
    [-3, -5, -4, -2, -1, -4, -5, -3],
    [-6, -6, -6, -6, -6, -6, -6, -6],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [6, 6, 6, 6, 6, 6, 6, 6],
    [3, 5, 4, 2, 1, 4, 5, 3],
];

//
// Structs
//
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

#[cfg_attr(feature = "wasm", wasm_bindgen, derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[allow(dead_code)]
    fn to_int(&self) -> isize {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum SquareColor {
    White,
    Black,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Castle {
    KingSideWhite,
    QueenSideWhite,
    KingSideBlack,
    QueenSideBlack,
}

impl Castle {
    #[allow(dead_code)]
    fn to_str(&self) -> &str {
        match self {
            Castle::KingSideWhite => CASTLE_KING_SIDE_WHITE,
            Castle::QueenSideWhite => CASTLE_QUEEN_SIDE_WHITE,
            Castle::KingSideBlack => CASTLE_KING_SIDE_BLACK,
            Castle::QueenSideBlack => CASTLE_QUEEN_SIDE_BLACK,
        }
    }
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Debug)]
pub struct Piece<'a> {
    id: isize,
    _type: PieceType,
    color: Color,
    icon: char,
    desc: &'a str,
}

pub const PIECES: [Piece; 13] = [
    Piece {
        icon: '♙',
        desc: PAWN_DESC,
        color: Color::Black,
        _type: PieceType::Pawn,
        id: -PAWN_ID,
    },
    Piece {
        icon: '♘',
        desc: KNIGHT_DESC,
        color: Color::Black,
        _type: PieceType::Knight,
        id: -KNIGHT_ID,
    },
    Piece {
        icon: '♗',
        desc: BISHOP_DESC,
        color: Color::Black,
        _type: PieceType::Bishop,
        id: -BISHOP_ID,
    },
    Piece {
        icon: '♖',
        desc: ROOK_DESC,
        color: Color::Black,
        _type: PieceType::Rook,
        id: -ROOK_ID,
    },
    Piece {
        icon: '♕',
        desc: QUEEN_DESC,
        color: Color::Black,
        _type: PieceType::Queen,
        id: -QUEEN_ID,
    },
    Piece {
        icon: '♔',
        desc: KING_DESC,
        color: Color::Black,
        _type: PieceType::King,
        id: -KING_ID,
    },
    Piece {
        icon: '.',
        desc: &" ",
        color: Color::White, // doesn't matter but must be set to avoid using Option<Color>
        _type: PieceType::Empty,
        id: EMPTY_SQUARE_ID,
    },
    Piece {
        icon: '♚',
        desc: KING_DESC,
        color: Color::White,
        _type: PieceType::King,
        id: KING_ID,
    },
    Piece {
        icon: '♛',
        desc: QUEEN_DESC,
        color: Color::White,
        _type: PieceType::Queen,
        id: QUEEN_ID,
    },
    Piece {
        icon: '♜',
        desc: ROOK_DESC,
        color: Color::White,
        _type: PieceType::Rook,
        id: ROOK_ID,
    },
    Piece {
        icon: '♝',
        desc: BISHOP_DESC,
        color: Color::White,
        _type: PieceType::Bishop,
        id: BISHOP_ID,
    },
    Piece {
        icon: '♞',
        desc: KNIGHT_DESC,
        color: Color::White,
        _type: PieceType::Knight,
        id: KNIGHT_ID,
    },
    Piece {
        icon: '♟',
        desc: PAWN_DESC,
        color: Color::White,
        _type: PieceType::Pawn,
        id: PAWN_ID,
    },
];

lazy_static! {
    pub static ref ID_TO_COLOR: HashMap<isize, Color> = {
        PIECES
            .iter()
            .map(|piece| (piece.id, piece.color))
            .collect::<HashMap<_, _>>()
    };
    pub static ref ID_TO_ICON: HashMap<isize, char> = {
        PIECES
            .iter()
            .map(|piece| (piece.id, piece.icon))
            .collect::<HashMap<_, _>>()
    };
    pub static ref ID_TO_TYPE: HashMap<isize, PieceType> = {
        PIECES
            .iter()
            .map(|piece| (piece.id, piece._type))
            .collect::<HashMap<_, _>>()
    };
    pub static ref ID_TO_DESC: HashMap<isize, &'static str> = {
        PIECES
            .iter()
            .map(|piece| (piece.id, piece.desc))
            .collect::<HashMap<_, _>>()
    };
}

//
// Types
//
pub type Board = [[isize; 8]; 8];
pub type Square = (isize, isize);
pub type Move = (Square, Square);

#[allow(dead_code)]
pub union MoveUnion {
    pub normal_move: Move,
    pub castle: Castle,
}
#[allow(dead_code)]
pub struct MoveStruct {
    pub is_castle: bool,
    pub data: MoveUnion,
}

//
// State struct
//
#[cfg_attr(feature = "wasm", wasm_bindgen, derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct State {
    board: [[isize; 8]; 8],
    pub current_player: Color,
    pub white_king_on_board: bool,
    pub black_king_on_board: bool,
    pub white_king_castle_is_possible: bool,
    pub white_queen_castle_is_possible: bool,
    pub black_king_castle_is_possible: bool,
    pub black_queen_castle_is_possible: bool,
    pub white_king_is_checked: bool,
    pub black_king_is_checked: bool,
}

impl State {
    #[allow(dead_code)]
    pub fn new(
        board: [[isize; 8]; 8],
        current_player: &str,
        white_king_castle_is_possible: bool,
        white_queen_castle_is_possible: bool,
        black_king_castle_is_possible: bool,
        black_queen_castle_is_possible: bool,
    ) -> Self {
        let _current_player: Color = player_string_to_enum(current_player);
        // check if kings are on board
        // this affects castling and king under attack checks
        let white_king_on_board = piece_is_on_board(&board, KING_ID);
        let black_king_on_board = piece_is_on_board(&board, -KING_ID);

        let mut _white_king_castle_is_possible = white_king_castle_is_possible;
        let mut _white_queen_castle_is_possible = white_queen_castle_is_possible;
        let mut _black_king_castle_is_possible = black_king_castle_is_possible;
        let mut _black_queen_castle_is_possible = black_queen_castle_is_possible;

        if white_king_on_board == false {
            _white_king_castle_is_possible = false;
            _white_queen_castle_is_possible = false;
        }
        if black_king_on_board == false {
            _black_king_castle_is_possible = false;
            _black_queen_castle_is_possible = false;
        }

        return Self {
            board,
            white_king_on_board,
            black_king_on_board,
            current_player: _current_player,
            white_king_castle_is_possible: _white_king_castle_is_possible,
            white_queen_castle_is_possible: _white_queen_castle_is_possible,
            black_king_castle_is_possible: _black_king_castle_is_possible,
            black_queen_castle_is_possible: _black_queen_castle_is_possible,
            white_king_is_checked: false,
            black_king_is_checked: false,
        };
    }

    #[allow(dead_code)]
    pub fn get_board(&self) -> [[isize; 8]; 8] {
        self.board
    }

    #[allow(dead_code)]
    fn update_player_king_checked(
        &mut self,
        player: Color,
        squares_under_attack_map: &HashMap<usize, bool>,
    ) {
        match player {
            Color::White => {
                self.white_king_is_checked =
                    _king_is_checked(&self, Color::White, squares_under_attack_map);
            }
            Color::Black => {
                self.black_king_is_checked =
                    _king_is_checked(&self, Color::Black, squares_under_attack_map);
            }
        }
    }

    #[cfg(feature = "python")]
    pub fn to_py_object(&self, dict: &PyDict) {
        dict.set_item(
            "white_king_castle_is_possible",
            self.white_king_castle_is_possible,
        )
        .unwrap();
        dict.set_item(
            "white_queen_castle_is_possible",
            self.white_queen_castle_is_possible,
        )
        .unwrap();
        dict.set_item(
            "black_king_castle_is_possible",
            self.black_king_castle_is_possible,
        )
        .unwrap();
        dict.set_item(
            "black_queen_castle_is_possible",
            self.black_queen_castle_is_possible,
        )
        .unwrap();
        dict.set_item("white_king_is_checked", self.white_king_is_checked)
            .unwrap();
        dict.set_item("black_king_is_checked", self.black_king_is_checked)
            .unwrap();

        let board: &[&[isize]] = &[
            &self.board[0],
            &self.board[1],
            &self.board[2],
            &self.board[3],
            &self.board[4],
            &self.board[5],
            &self.board[6],
            &self.board[7],
        ];

        dict.set_item("board", array2d_to_vec2d(board)).unwrap();
        let current_player: &str = player_enum_to_string(&self.current_player);
        dict.set_item("current_player", current_player).unwrap();
    }
}

#[allow(dead_code)]
pub fn render_state(state: &State) {
    render_board(&state.board);
}

pub fn render_board(board: &[[isize; 8]; 8]) {
    print!("\n   ------------------------");
    for (j, row) in board.iter().enumerate() {
        print!("\n{} |", 8 - j);
        for piece_id in row.iter() {
            let piece_icon = ID_TO_ICON.get(piece_id);
            print!(" {} ", piece_icon.unwrap().to_string());
        }
        print!("|");
    }
    println!("\n   ------------------------");
    println!("    a  b  c  d  e  f  g  h");
}

#[allow(dead_code)]
fn array2d_to_vec2d(arr: &[&[isize]]) -> Vec<Vec<isize>> {
    let mut vec: Vec<Vec<isize>> = Vec::new();
    for &row in arr.iter() {
        vec.push(row.iter().cloned().collect());
    }
    return vec;
}

fn player_string_to_enum(player: &str) -> Color {
    let mut _player: Color = Color::White;
    match player {
        "WHITE" => {
            _player = Color::White;
        }
        "BLACK" => {
            _player = Color::Black;
        }
        _ => {
            println!("Invalid Color. Must be 'WHITE' or 'BLACK'");
        }
    }
    return _player;
}

#[allow(dead_code)]
fn player_enum_to_string<'a>(player: &Color) -> &'a str {
    let mut _player: &str = "";
    match player {
        Color::White => {
            _player = "WHITE";
        }
        _ => _player = "BLACK",
    }
    return _player;
}

//
// CORE LOGIC
// ---------------------------------------------------------
// ---------------------------------------------------------

// shortcut
#[allow(dead_code)]
pub fn get_all_possible_moves(
    state: &State,
    player: Color,
    attack: bool,
) -> (Vec<Move>, Vec<Castle>) {
    // squares under attack
    let other_player: Color = get_other_player(player);
    let mut squares_under_attack_map: HashMap<usize, bool> = HashMap::new();
    if attack != true {
        squares_under_attack_map = get_squares_under_attack_by_player(&state, other_player);
    }
    //
    return _get_all_possible_moves(state, player, attack, &squares_under_attack_map);
}

// get all moves (normal + castles)
pub fn _get_all_possible_moves(
    state: &State,
    player: Color,
    attack: bool,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> (Vec<Move>, Vec<Castle>) {
    let moves: Vec<Move> = _get_possible_moves(state, player, attack, squares_under_attack_map);
    let castle_moves: Vec<Castle> =
        _get_possible_castle_moves(state, player, attack, &squares_under_attack_map);
    return (moves, castle_moves);
}

// shortcut function
#[allow(dead_code)]
pub fn get_possible_moves(state: &State, player: Color, attack: bool) -> Vec<Move> {
    // squares under attack
    let other_player: Color = get_other_player(player);
    let mut squares_under_attack_map: HashMap<usize, bool> = HashMap::new();
    if attack != true {
        squares_under_attack_map = get_squares_under_attack_by_player(&state, other_player);
    }
    //
    return _get_possible_moves(state, player, attack, &squares_under_attack_map);
}

// function to be used in the Python api func
pub fn _get_possible_moves(
    state: &State,
    player: Color,
    attack: bool,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> Vec<Move> {
    // calculate possible moves
    let mut moves: Vec<Move> = vec![];

    for (_i, row) in state.board.iter().enumerate() {
        for (_j, piece_id) in row.iter().enumerate() {
            let i = _i as isize;
            let j = _j as isize;
            // empty square
            if *piece_id == 0 {
                continue;
            }
            // other player's piece
            let piece_color: Color = *ID_TO_COLOR.get(piece_id).unwrap();
            if piece_color != player {
                continue;
            }
            // player piece
            let piece_type = ID_TO_TYPE[piece_id];
            match piece_type {
                PieceType::King => {
                    let _moves: Vec<Move> =
                        king_moves(&state, player, (i, j), squares_under_attack_map, attack);
                    moves.extend_from_slice(&_moves);
                }
                PieceType::Queen => {
                    let _moves: Vec<Move> = queen_moves(&state, player, (i, j), attack);
                    moves.extend_from_slice(&_moves);
                }
                PieceType::Rook => {
                    let _moves: Vec<Move> = rook_moves(&state, player, (i, j), attack);
                    moves.extend_from_slice(&_moves);
                }
                PieceType::Bishop => {
                    let _moves: Vec<Move> = bishop_moves(&state, player, (i, j), attack);
                    moves.extend_from_slice(&_moves);
                }
                PieceType::Knight => {
                    let _moves: Vec<Move> = knight_moves(&state, player, (i, j), attack);
                    moves.extend_from_slice(&_moves);
                }
                PieceType::Pawn => {
                    let _moves: Vec<Move> = pawn_moves(&state, player, (i, j), attack);
                    moves.extend_from_slice(&_moves);
                }
                _ => {}
            }
        }
    }

    if attack == true {
        return moves;
    }

    // Filter out moves that leave the king checked
    moves.retain(|_move: &Move| !move_leaves_king_checked(state, player, *_move));
    return moves;
}

// shortcut function
#[allow(dead_code)]
pub fn get_possible_castle_moves(state: &State, player: Color, attack: bool) -> Vec<Castle> {
    // squares under attack
    let other_player: Color = get_other_player(player);
    let mut squares_under_attack_map: HashMap<usize, bool> = HashMap::new();
    if attack != true {
        squares_under_attack_map = get_squares_under_attack_by_player(&state, other_player);
    }
    //
    return _get_possible_castle_moves(state, player, attack, &squares_under_attack_map);
}

// function to be used in the Python api func
pub fn _get_possible_castle_moves(
    state: &State,
    player: Color,
    attack: bool,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> Vec<Castle> {
    // calculate possible castling moves
    let mut castle_moves: Vec<Castle> = vec![];

    // castling aren not attacking moves
    if attack == true {
        return castle_moves;
    }

    // King not present on the board (for testing pruposes)
    if (player == Color::White && !state.white_king_on_board == true)
        || (player == Color::Black && !state.black_king_on_board == true)
    {
        return castle_moves;
    }

    if (player == Color::White
        && (state.white_king_castle_is_possible == true
            || state.white_queen_castle_is_possible == true))
        || (player == Color::Black
            && (state.black_king_castle_is_possible == true
                || state.black_queen_castle_is_possible == true))
    {
        castle_moves = calc_castle_moves(state, player, squares_under_attack_map);
    }

    return castle_moves;
}

fn move_leaves_king_checked(state: &State, player: Color, _move: Move) -> bool {
    // skip king moves
    // let _from = (_move.0 .0 as usize, _move.0 .1 as usize);
    // if (player == Color::White && state.board[_from.0][_from.1] == KING_ID)
    //     || (player == Color::Black && state.board[_from.0][_from.1] == -KING_ID)
    // {
    //     return false;
    // }
    let move_struct = MoveStruct {
        is_castle: false,
        data: MoveUnion { normal_move: _move },
    };
    let _next_state = next_state(state, player, move_struct);
    return king_is_checked(&_next_state, player);
}

pub fn king_is_checked(state: &State, player: Color) -> bool {
    let other_player = get_other_player(player);
    let squares_under_attack_map = get_squares_under_attack_by_player(state, other_player);
    return _king_is_checked(state, player, &squares_under_attack_map);
}

fn _king_is_checked(
    state: &State,
    player: Color,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> bool {
    // TODO:
    // King not present on the board (for testing pruposes)
    let mut king_square: Option<Square> = None;
    let king_id = KING_ID * player.to_int();

    for (_i, row) in state.board.iter().enumerate() {
        for (_j, piece_id) in row.iter().copied().enumerate() {
            if piece_id == king_id {
                let i = _i as isize;
                let j = _j as isize;
                king_square = Some((i, j));
                break;
            }
        }
    }

    match king_square {
        None => {
            return false;
        }
        Some(square) => {
            let square_flat = square_tuple_to_flat(square);
            match squares_under_attack_map.get(&square_flat) {
                Some(&_) => return true,
                None => return false,
            }
        }
    }
}

fn get_squares_under_attack_by_player(state: &State, player: Color) -> HashMap<usize, bool> {
    let mut squares_under_attack_map: HashMap<usize, bool> = HashMap::new();
    // bug

    let moves = _get_possible_moves(&state, player, true, &squares_under_attack_map);
    for _move in moves.iter() {
        let square_flat = square_tuple_to_flat(_move.1);
        squares_under_attack_map.insert(square_flat, true);
    }
    return squares_under_attack_map;
}

pub fn next_state(state: &State, player: Color, move_struct: MoveStruct) -> State {
    let mut new_state = state.clone();

    unsafe {
        match move_struct {
            MoveStruct {
                is_castle: false,
                data: MoveUnion { normal_move },
            } => {
                let _from = (normal_move.0 .0 as usize, normal_move.0 .1 as usize);
                let _to = (normal_move.1 .0 as usize, normal_move.1 .1 as usize);
                let piece_to_move = new_state.board[_from.0][_from.1];
                if piece_to_move == 0 {
                    panic!("Bad move - piece is empty !!!");
                }
                new_state.board[_from.0][_from.1] = 0;
                new_state.board[_to.0][_to.1] = piece_to_move;

                // Pawn becomes Queen
                let piece_type = *ID_TO_TYPE.get(&piece_to_move).unwrap();
                if piece_type == PieceType::Pawn {
                    if (player == Color::White && _to.0 == 0)
                        || (player == Color::Black && _to.0 == 7)
                    {
                        new_state.board[_to.0][_to.1] = QUEEN_ID * player.to_int();
                    }
                }

                // Keep track if castling is still possible
                if piece_to_move == KING_ID {
                    if player == Color::White {
                        new_state.white_king_castle_is_possible = false;
                        new_state.white_queen_castle_is_possible = false;
                    } else {
                        new_state.black_king_castle_is_possible = false;
                        new_state.black_queen_castle_is_possible = false;
                    }
                } else if piece_to_move == ROOK_ID {
                    if _from.1 == 0 {
                        if player == Color::White {
                            new_state.white_queen_castle_is_possible = false;
                        } else {
                            new_state.black_queen_castle_is_possible = false;
                        }
                    } else if _from.1 == 7 {
                        if player == Color::White {
                            new_state.white_king_castle_is_possible = false;
                        } else {
                            new_state.black_king_castle_is_possible = false;
                        }
                    }
                }
            }
            MoveStruct {
                is_castle: true,
                data: MoveUnion { castle },
            } => match castle {
                Castle::KingSideWhite => {
                    new_state.board[7][4] = EMPTY_SQUARE_ID;
                    new_state.board[7][5] = ROOK_ID;
                    new_state.board[7][6] = KING_ID;
                    new_state.board[7][7] = EMPTY_SQUARE_ID;
                    new_state.white_king_castle_is_possible = false;
                    new_state.white_queen_castle_is_possible = false;
                }
                Castle::QueenSideWhite => {
                    new_state.board[7][0] = EMPTY_SQUARE_ID;
                    new_state.board[7][1] = EMPTY_SQUARE_ID;
                    new_state.board[7][2] = KING_ID;
                    new_state.board[7][3] = ROOK_ID;
                    new_state.board[7][4] = EMPTY_SQUARE_ID;
                    new_state.white_king_castle_is_possible = false;
                    new_state.white_queen_castle_is_possible = false;
                }
                Castle::KingSideBlack => {
                    new_state.board[0][4] = EMPTY_SQUARE_ID;
                    new_state.board[0][5] = -ROOK_ID;
                    new_state.board[0][6] = -KING_ID;
                    new_state.board[0][7] = EMPTY_SQUARE_ID;
                    new_state.black_king_castle_is_possible = false;
                    new_state.black_queen_castle_is_possible = false;
                }
                Castle::QueenSideBlack => {
                    new_state.board[0][0] = EMPTY_SQUARE_ID;
                    new_state.board[0][1] = EMPTY_SQUARE_ID;
                    new_state.board[0][2] = -KING_ID;
                    new_state.board[0][3] = -ROOK_ID;
                    new_state.board[0][4] = EMPTY_SQUARE_ID;
                    new_state.black_king_castle_is_possible = false;
                    new_state.black_queen_castle_is_possible = false;
                }
            },
        }
    }

    // change player
    let other_player = get_other_player(player);
    new_state.current_player = other_player;
    // render_state(&new_state);

    return new_state;
}

// PIECE MOVEMENTS
// ---------------------------------------------------------
// ---------------------------------------------------------
fn king_moves(
    state: &State,
    player: Color,
    coords: Square,
    squares_under_attack_map: &HashMap<usize, bool>,
    attack: bool,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let steps: [Square; 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for step in steps.iter() {
        let square: Square = (coords.0 + step.0, coords.1 + step.1);
        if attack == true {
            let add = king_attacking_move(state, player, square, squares_under_attack_map);
            if add == true {
                moves.push((coords, square));
            }
        } else {
            let add = king_playable_move(state, player, square, squares_under_attack_map);
            if add == true {
                moves.push((coords, square));
            }
        }
    }
    return moves;
}

fn queen_moves(state: &State, player: Color, coords: Square, attack: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let moves_rook: Vec<Move> = rook_moves(state, player, coords, attack);
    moves.extend_from_slice(&moves_rook);
    let moves_bishop: Vec<Move> = bishop_moves(state, player, coords, attack);
    moves.extend_from_slice(&moves_bishop);
    return moves;
}

fn rook_moves(state: &State, player: Color, coords: Square, attack: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let steps: [Square; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for step in steps.iter() {
        let _moves: Vec<Move> = iterativesteps(state, player, coords, *step, attack);
        moves.extend_from_slice(&_moves)
    }
    return moves;
}

fn bishop_moves(state: &State, player: Color, coords: Square, attack: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let steps: [Square; 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for step in steps.iter() {
        let _moves: Vec<Move> = iterativesteps(state, player, coords, *step, attack);
        moves.extend_from_slice(&_moves)
    }
    return moves;
}

fn iterativesteps(
    state: &State,
    player: Color,
    coords: Square,
    step: Square,
    attack: bool,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let mut k: isize = 1;
    loop {
        let square = (coords.0 + k * step.0, coords.1 + k * step.1);
        if attack == true {
            let (add, stop) = attacking_move(state, player, square);
            if add == true {
                moves.push((coords, square));
            }
            if stop == true {
                break;
            } else {
                k += 1;
            }
        } else {
            let (add, stop) = playable_move(state, player, square);
            if add == true {
                moves.push((coords, square));
            }
            if stop == true {
                break;
            } else {
                k += 1;
            }
        }
    }
    return moves;
}

fn knight_moves(state: &State, player: Color, coords: Square, attack: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let steps: [Square; 8] = [
        (-2, -1),
        (-2, 1),
        (2, -1),
        (2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
    ];
    for step in steps.iter() {
        let square = (coords.0 + step.0, coords.1 + step.1);
        if attack == true {
            let (add, _) = attacking_move(state, player, square);
            if add == true {
                moves.push((coords, square));
            }
        } else {
            let (add, _) = playable_move(state, player, square);
            if add == true {
                moves.push((coords, square));
            }
        }
    }
    return moves;
}

fn pawn_moves(state: &State, player: Color, coords: Square, attack: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let player_int: isize = player.to_int();
    let attack_squares: [Square; 2] = [
        (coords.0 - player_int, coords.1 + 1),
        (coords.0 - player_int, coords.1 - 1),
    ];
    let one_step_square: Square = (coords.0 + (1 * -player_int), coords.1);
    let two_step_square: Square = (coords.0 + (2 * -player_int), coords.1);

    if attack == true {
        for square in attack_squares.iter().cloned() {
            if square_is_on_board(square) && !is_king_from_player(state, player, square) {
                moves.push((coords, square));
            }
        }
    } else {
        {
            let x = one_step_square.0 as usize;
            let y = one_step_square.1 as usize;
            if square_is_on_board(one_step_square) && state.board[x][y] == 0 {
                moves.push((coords, one_step_square));
            }
        }
        {
            let x = two_step_square.0 as usize;
            let y = two_step_square.1 as usize;
            let x2 = one_step_square.0 as usize;
            let y2 = one_step_square.1 as usize;
            if square_is_on_board(two_step_square) {
                if (player == Color::White && coords.0 == 6)
                    || (player == Color::Black && coords.0 == 1)
                {
                    if state.board[x][y] == 0 && state.board[x2][y2] == 0 {
                        moves.push((coords, two_step_square));
                    }
                }
            }
        }
        for square in attack_squares.iter().cloned() {
            if square_is_on_board(square) && is_piece_from_other_player(state, player, square) {
                moves.push((coords, square));
            }
        }
        // TODO: implement en-passant pawn capture
        //
    }
    return moves;
}

fn calc_castle_moves(
    state: &State,
    player: Color,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> Vec<Castle> {
    let mut castle_moves: Vec<Castle> = vec![];

    if player == Color::White {
        // White castle Queen side
        let rook: (usize, usize) = (7, 0);
        let empty_3: (usize, usize) = (7, 1);
        let empty_2: (usize, usize) = (7, 2);
        let empty_1: (usize, usize) = (7, 3);
        let king: (usize, usize) = (7, 4);
        let _king = (king.0 as isize, king.1 as isize);
        let _empty_1 = (empty_1.0 as isize, empty_1.1 as isize);
        let _empty_2 = (empty_2.0 as isize, empty_2.1 as isize);
        if state.board[rook.0][rook.1] == ROOK_ID
            && state.board[empty_3.0][empty_3.1] == EMPTY_SQUARE_ID
            && state.board[empty_2.0][empty_2.1] == EMPTY_SQUARE_ID
            && state.board[empty_1.0][empty_1.1] == EMPTY_SQUARE_ID
            && state.board[king.0][king.1] == KING_ID
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_king))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_1))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_2))
        {
            castle_moves.push(Castle::QueenSideWhite);
        }

        // White castle King side
        let king: (usize, usize) = (7, 4);
        let empty_1: (usize, usize) = (7, 5);
        let empty_2: (usize, usize) = (7, 6);
        let rook: (usize, usize) = (7, 7);
        let _king = (king.0 as isize, king.1 as isize);
        let _empty_1 = (empty_1.0 as isize, empty_1.1 as isize);
        let _empty_2 = (empty_2.0 as isize, empty_2.1 as isize);
        if state.board[rook.0][rook.1] == ROOK_ID
            && state.board[empty_2.0][empty_2.1] == EMPTY_SQUARE_ID
            && state.board[empty_1.0][empty_1.1] == EMPTY_SQUARE_ID
            && state.board[king.0][king.1] == KING_ID
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_king))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_1))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_2))
        {
            castle_moves.push(Castle::KingSideWhite);
        }
    } else {
        // Black castle Queen side
        let rook: (usize, usize) = (0, 0);
        let empty_3: (usize, usize) = (0, 1);
        let empty_2: (usize, usize) = (0, 2);
        let empty_1: (usize, usize) = (0, 3);
        let king: (usize, usize) = (0, 4);
        let _king = (king.0 as isize, king.1 as isize);
        let _empty_1 = (empty_1.0 as isize, empty_1.1 as isize);
        let _empty_2 = (empty_2.0 as isize, empty_2.1 as isize);
        if state.board[rook.0][rook.1] == ROOK_ID
            && state.board[empty_3.0][empty_3.1] == EMPTY_SQUARE_ID
            && state.board[empty_2.0][empty_2.1] == EMPTY_SQUARE_ID
            && state.board[empty_1.0][empty_1.1] == EMPTY_SQUARE_ID
            && state.board[king.0][king.1] == KING_ID
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_king))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_1))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_2))
        {
            castle_moves.push(Castle::QueenSideBlack);
        }

        // Black castle King side
        let king: (usize, usize) = (0, 4);
        let empty_1: (usize, usize) = (0, 5);
        let empty_2: (usize, usize) = (0, 6);
        let rook: (usize, usize) = (0, 7);
        let _king = (king.0 as isize, king.1 as isize);
        let _empty_1 = (empty_1.0 as isize, empty_1.1 as isize);
        let _empty_2 = (empty_2.0 as isize, empty_2.1 as isize);
        if state.board[rook.0][rook.1] == ROOK_ID
            && state.board[empty_2.0][empty_2.1] == EMPTY_SQUARE_ID
            && state.board[empty_1.0][empty_1.1] == EMPTY_SQUARE_ID
            && state.board[king.0][king.1] == KING_ID
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_king))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_1))
            && !squares_under_attack_map.contains_key(&square_tuple_to_flat(_empty_2))
        {
            castle_moves.push(Castle::KingSideBlack);
        }
    }

    return castle_moves;
}

///
/// return squares to which a piece can move
/// - empty squares
/// - opponent pieces (excluding king)
/// => return [<bool> playable, <bool> stop_iteration]
fn playable_move(state: &State, player: Color, square: Square) -> (bool, bool) {
    let other_player = get_other_player(player);
    if !square_is_on_board(square) {
        return (false, true);
    }
    if square_is_empty(state, square) {
        return (true, false);
    }
    if is_piece_from_player(state, player, square) {
        return (false, true);
    }
    if is_piece_from_player(state, other_player, square) {
        return (true, true);
    }
    if is_king_from_player(state, other_player, square) {
        return (false, true);
    }
    panic!("PLAYABLE MOVE ERROR");
}

///
/// return squares that are attacked or defended
/// - empty squares
/// - opponent pieces (opponent king is ignored)
/// - own pieces
/// => return [<bool> playable, <bool> stop_iteration]
fn attacking_move(state: &State, player: Color, square: Square) -> (bool, bool) {
    let other_player = get_other_player(player);
    if !square_is_on_board(square) {
        return (false, true);
    }
    if square_is_empty(state, square) {
        return (true, false);
    }
    if is_piece_from_player(state, player, square)
        || is_piece_from_player(state, other_player, square)
        || is_king_from_player(state, other_player, square)
    {
        return (true, true);
    }
    panic!("ATTACKING MOVE ERROR");
}

///
/// return squares to which the king can move,
/// i.e. unattacked squares that can be:
/// - empty squares
/// - opponent pieces (excluding king)
/// If opponent king is encountered, then there's a problem...
/// => return <bool> is_playable
fn king_playable_move(
    state: &State,
    player: Color,
    square: Square,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> bool {
    let other_player = get_other_player(player);
    if !square_is_on_board(square) {
        return false;
    }

    let square_flat = square_tuple_to_flat(square);
    match squares_under_attack_map.get(&square_flat) {
        Some(&_) => return false,
        None => {}
    }

    if square_is_empty(state, square) || is_piece_from_player(state, other_player, square) {
        return true;
    }
    if is_piece_from_player(state, player, square) {
        return false;
    }
    if is_king_from_player(state, other_player, square) {
        panic!("KINGS NEXT TO EACH OTHER ERROR");
    }
    panic!("KING PLAYABLE MOVE ERROR");
}

///
/// return all the squares that the king can attack, except:
/// - squares outside the board
/// If opponent king is encountered, then there's a problem...
/// => return <bool> is_playable
fn king_attacking_move(
    state: &State,
    player: Color,
    square: Square,
    squares_under_attack_map: &HashMap<usize, bool>,
) -> bool {
    let other_player = get_other_player(player);
    if !square_is_on_board(square) {
        return false;
    }

    let square_flat = square_tuple_to_flat(square);
    match squares_under_attack_map.get(&square_flat) {
        Some(&_) => return false,
        _ => {}
    }

    if square_is_empty(state, square)
        || is_piece_from_player(state, player, square)
        || is_piece_from_player(state, other_player, square)
    {
        return true;
    }
    if is_king_from_player(state, other_player, square) {
        panic!("KINGS NEXT TO EACH OTHER ERROR");
    }
    panic!("KING PLAYABLE MOVE ERROR");
}

// Utility FUNCTIONS
// ---------------------------------------------------------
// ---------------------------------------------------------
fn get_other_player(player: Color) -> Color {
    match player {
        Color::White => {
            return Color::Black;
        }
        _ => {
            return Color::White;
        }
    }
}

fn square_is_on_board(square: Square) -> bool {
    return !(square.0 < 0 || square.0 > 7 || square.1 < 0 || square.1 > 7);
}

fn square_is_empty(state: &State, square: Square) -> bool {
    let row = square.0 as usize;
    let col = square.1 as usize;
    let piece_id = state.board[row][col];
    return piece_id == 0;
}

fn is_piece_from_player(state: &State, player: Color, square: Square) -> bool {
    let row = square.0 as usize;
    let col = square.1 as usize;
    let piece_id = state.board[row][col];
    if piece_id == 0 {
        return false;
    }
    let piece_color = *ID_TO_COLOR.get(&piece_id).unwrap();
    return player == piece_color;
}

fn is_piece_from_other_player(state: &State, player: Color, square: Square) -> bool {
    let other_player = get_other_player(player);
    return is_piece_from_player(state, other_player, square);
}

fn is_king_from_player(state: &State, player: Color, square: Square) -> bool {
    let board = state.board;
    let row = square.0 as usize;
    let col = square.1 as usize;
    let piece_id = board[row][col];
    let piece_type = *ID_TO_TYPE.get(&piece_id).unwrap();
    if piece_type != PieceType::King {
        return false;
    }
    let piece_color = *ID_TO_COLOR.get(&piece_id).unwrap();
    return piece_color == player;
}

// fn is_king_from_other_player(state: &State, player: Color, square: Square) -> bool {
//     let other_player = get_other_player(player);
//     return is_king_from_player(state, other_player, square);
// }

fn square_tuple_to_flat(square: Square) -> usize {
    let square_flat = square.0 * 8 + square.1;
    square_flat as usize
}

// fn square_flat_to_tuple(square_flat: usize) -> Square {
//     let row = square_flat / 8;
//     let col = square_flat % 8;
//     (row as isize, col as isize)
// }

#[allow(dead_code)]
#[cfg(feature = "python")]
fn convert_py_state<'a>(_py: Python<'a>, state_py: &'a PyDict) -> PyResult<State> {
    let board: Board = state_py.get_item("board").unwrap().extract()?;
    let current_player: &str = state_py.get_item("current_player").unwrap().extract()?;
    let white_king_castle_is_possible: bool = state_py
        .get_item("white_king_castle_is_possible")
        .unwrap()
        .extract()?;
    let white_queen_castle_is_possible: bool = state_py
        .get_item("white_queen_castle_is_possible")
        .unwrap()
        .extract()?;
    let black_king_castle_is_possible: bool = state_py
        .get_item("black_king_castle_is_possible")
        .unwrap()
        .extract()?;
    let black_queen_castle_is_possible: bool = state_py
        .get_item("black_queen_castle_is_possible")
        .unwrap()
        .extract()?;

    // create state
    let state = State::new(
        board,
        current_player,
        white_king_castle_is_possible,
        white_queen_castle_is_possible,
        black_king_castle_is_possible,
        black_queen_castle_is_possible,
    );
    return Ok(state);
}

#[allow(dead_code)]
pub fn convert_move_to_string(_move: Move) -> String {
    let _from = (_move.0 .0 as usize, _move.0 .1 as usize);
    let _to = (_move.1 .0 as usize, _move.1 .1 as usize);
    let cols = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let from_str = format!(
        "{}{}{}{}",
        cols[_from.1],
        8 - _from.0,
        cols[_to.1],
        8 - _to.0
    );
    return from_str;
}

#[allow(dead_code)]
fn convert_castle_move_to_string(castle_move: Castle) -> String {
    castle_move.to_string()
}

// fn convert_move_union_to_string(move_struct: MoveStruct) -> String {
//     unsafe {
//         match move_struct {
//             MoveStruct {
//                 is_castle: false,
//                 data: MoveUnion { normal_move },
//             } => convert_move_to_string(normal_move),
//             MoveStruct {
//                 is_castle: true,
//                 data: MoveUnion { castle },
//             } => convert_castle_move_to_string(castle),
//         }
//     }
// }

#[allow(dead_code)]
fn convert_move_to_type(_move: &str) -> MoveStruct {
    let letters: HashMap<&str, isize> = [
        ("a", 0),
        ("b", 1),
        ("c", 2),
        ("d", 3),
        ("e", 4),
        ("f", 5),
        ("g", 6),
        ("h", 7),
    ]
    .iter()
    .copied()
    .collect();

    match _move {
        CASTLE_KING_SIDE_WHITE => {
            return MoveStruct {
                is_castle: true,
                data: MoveUnion {
                    castle: Castle::KingSideWhite,
                },
            };
        }
        CASTLE_QUEEN_SIDE_WHITE => {
            return MoveStruct {
                is_castle: true,
                data: MoveUnion {
                    castle: Castle::QueenSideWhite,
                },
            };
        }
        CASTLE_KING_SIDE_BLACK => {
            return MoveStruct {
                is_castle: true,
                data: MoveUnion {
                    castle: Castle::KingSideBlack,
                },
            };
        }
        CASTLE_QUEEN_SIDE_BLACK => {
            return MoveStruct {
                is_castle: true,
                data: MoveUnion {
                    castle: Castle::QueenSideBlack,
                },
            };
        }
        _ => {
            let _from_0: isize = _move[1..2].parse::<isize>().unwrap();
            let _from_1: &str = &_move[0..1];
            let _to_0: isize = _move[3..4].parse::<isize>().unwrap();
            let _to_1: &str = &_move[2..3];
            let _from = (8 - _from_0, *letters.get(_from_1).unwrap());
            let _to = (8 - _to_0, *letters.get(_to_1).unwrap());
            let _move: Move = (_from, _to);
            return MoveStruct {
                is_castle: false,
                data: MoveUnion { normal_move: _move },
            };
        }
    }
}

#[allow(dead_code)]
pub fn insufficient_material(board: &Board) -> bool {
    let mut sq_color = 0;
    let mut num_pieces = 0;
    let mut bishops: Vec<i32> = Vec::new();
    let mut pieces: HashMap<isize, isize> = HashMap::new();
    for row in board.iter() {
        for p_id in row.iter() {
            sq_color = (sq_color + 1) % 2;
            let piece = p_id;
            if *piece != 0 {
                let count = pieces.entry(piece.abs()).or_insert(0);
                *count += 1;
                if piece.abs() == BISHOP_ID {
                    bishops.push(sq_color);
                }
                num_pieces += 1;
            }
        }
    }
    // k vs. k
    if num_pieces == 2 {
        return true;
    }
    // k vs. kn .... or .... k vs. kb
    match num_pieces {
        3 => match (pieces.get(&BISHOP_ID), pieces.get(&KNIGHT_ID)) {
            (Some(1), ..) | (.., Some(1)) => return true,
            _ => (),
        },
        _ => (),
    }
    // kb vs. kb where any number of bishops are all on the same color
    match pieces.get(&BISHOP_ID) {
        Some(x) => match num_pieces - x {
            2 => {
                let mut sum = 0;
                let len = bishops.len();
                for elem in bishops {
                    sum += elem;
                }
                if sum == 0 || sum == len as i32 {
                    return true;
                }
            }
            _ => (),
        },
        None => (),
    }

    false
}

fn piece_is_on_board(board: &[[isize; 8]; 8], piece_id: isize) -> bool {
    for row in board.iter() {
        for p_id in row.iter() {
            if *p_id == piece_id {
                return true;
            }
        }
    }
    return false;
}

#[allow(dead_code)]
pub fn is_game_over(states: &Vec<State>, state: &State, player: Color) -> u8 {
    if checkmate(state, player) {
        return 1;
    }
    if in_stalemate(state, player) {
        return 2;
    }
    if in_threefold_repetition(states) {
        return 3;
    }
    if insufficient_material(&state.get_board()) {
        return 4;
    }
    return 0;
}

#[allow(dead_code)]
pub fn in_stalemate(state: &State, player: Color) -> bool {
    if !king_is_checked(state, player) && get_possible_moves(state, player, false).len() == 0 {
        return true;
    }
    false
}

#[allow(dead_code)]
pub fn checkmate(state: &State, player: Color) -> bool {
    if king_is_checked(state, player) && get_possible_moves(state, player, false).len() == 0 {
        return true;
    }
    false
}

#[allow(dead_code)]
pub fn in_threefold_repetition(states: &Vec<State>) -> bool {
    let mut states_hash: HashMap<State, isize> = HashMap::new();
    for state in states.iter() {
        let count = states_hash.entry(*state).or_insert(0);
        *count += 1;
        if *count == 3 {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn update_state(state: &mut State) {
    // white
    let squares_under_attack_by_black = get_squares_under_attack_by_player(state, Color::Black);
    state.update_player_king_checked(Color::White, &squares_under_attack_by_black);
    // black
    let squares_under_attack_by_white = get_squares_under_attack_by_player(state, Color::White);
    state.update_player_king_checked(Color::Black, &squares_under_attack_by_white);
}

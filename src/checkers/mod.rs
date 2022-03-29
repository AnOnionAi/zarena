pub mod python;

use std::io::stdout;

mod checkers;

use checkers::{
    Board, BoardPosition, EmptyTile, Game, GameState, JumpMove, KingPiece, ManPiece, MoveError,
    OccupiedTile, SimpleMove, Tile,
};

use crate::checkers::checkers::PieceType;

mod util;

fn apply_positions_as_move(
    game: &mut Game,
    positions: Vec<BoardPosition>,
) -> Result<GameState, MoveError> {
    if positions.len() == 2 {
        let start = positions[0];
        let end = positions[1];

        let row_diff = util::absolute_diff(start.row, end.row);
        let col_diff = util::absolute_diff(start.column, end.column);

        if row_diff == 1 && col_diff == 1 {
            game.apply_simple_move(SimpleMove::new(
                start.row,
                start.column,
                end.row,
                end.column,
            ))
        } else {
            game.apply_jump_move(positions)
        }
    } else {
        game.apply_jump_move(positions)
    }
}

enum PlayerColor {
    Red,
    Black,
}

fn player_id_to_color(player_id: u32) -> PlayerColor {
    match player_id {
        1 => PlayerColor::Red,
        2 => PlayerColor::Black,
        _ => unreachable!(),
    }
}

fn player_color_to_name(color: PlayerColor) -> &'static str {
    match color {
        PlayerColor::Red => "Red",
        PlayerColor::Black => "Black",
    }
}

struct IntBoardValues {
    empty: u8,
    man_1: u8,
    king_1: u8,
    man_2: u8,
    king_2: u8,
}

struct Checkers {
    game: Game,
}

impl Checkers {
    fn new() -> Self {
        Checkers { game: Game::new() }
    }

    fn action_to_positions(&self, action: &usize) -> Vec<BoardPosition> {
        let mut positions = Vec::new();
        let from = action / 32;
        let to = action % 32;
        let x0 = from / 4;
        let y0 = from % 4;
        positions.push(BoardPosition::new(
            x0,
            y0 * 2 + if x0 % 2 == 0 { 0 } else { 1 },
        ));
        let x1 = to / 4;
        let y1 = to % 4;
        positions.push(BoardPosition::new(
            x1,
            y1 * 2 + if x1 % 2 == 0 { 0 } else { 1 },
        ));
        positions
    }

    fn positions_to_action(&self, positions: &Vec<BoardPosition>) -> usize {
        let from = positions[0].row * 4
            + (positions[0].column - if positions[0].row % 2 == 0 { 0 } else { 1 }) / 2;
        let to = positions[1].row * 4
            + (positions[1].column - if positions[1].row % 2 == 0 { 0 } else { 1 }) / 2;
        from * 32 + to
    }

    fn simple_move_to_action(&self, simple_move: &SimpleMove) -> usize {
        let mut positions = Vec::new();
        positions.push(BoardPosition::new(
            simple_move.from_row(),
            simple_move.from_column(),
        ));
        positions.push(BoardPosition::new(
            simple_move.to_row(),
            simple_move.to_column(),
        ));
        self.positions_to_action(&positions)
    }

    fn jump_move_to_action(&self, jump_move: &JumpMove) -> usize {
        let mut positions = Vec::new();
        positions.push(BoardPosition::new(
            jump_move.from_row(),
            jump_move.from_column(),
        ));
        positions.push(BoardPosition::new(
            jump_move.get_jumps()[0].from_row(),
            jump_move.get_jumps()[0].from_column(),
        ));
        self.positions_to_action(&positions)
    }

    fn step(&mut self, action: usize) -> (Vec<Vec<Vec<u8>>>, f32, bool) {
        let positions = self.action_to_positions(&action);
        let move_result = apply_positions_as_move(&mut self.game, positions);
        let reward = match move_result {
            Ok(game_state) => match game_state {
                GameState::InProgress => 0f32,
                GameState::GameOver { winner_id: _ } => 1f32,
            },
            Err(e) => match e {
                MoveError::InvalidMove => panic!("\n *** Illegal move"),
                MoveError::ShouldHaveJumped => panic!("\n *** Must take jump"),
            },
        };
        (self.get_observation(), reward, self.game.is_game_over())
    }

    fn legal_actions(&self) -> Vec<usize> {
        let available_simple_moves = &self.game.get_available_simple_moves();
        let available_jump_moves = &self.game.get_available_jump_moves();
        if available_jump_moves.is_empty() {
            available_simple_moves
                .iter()
                .map(|positions| self.simple_move_to_action(positions))
                .collect()
        } else {
            available_jump_moves
                .iter()
                .map(|positions| self.jump_move_to_action(positions))
                .collect()
        }
    }

    fn to_play(&self) -> u8 {
        (self.game.current_player().id - 1) as u8
    }

    fn reset(&mut self) -> Vec<Vec<Vec<u8>>> {
        self.game = Game::new();
        self.get_observation()
    }

    fn print(&self) {
        let mut writer = stdout();
        checkers::print_board(&mut writer, self.game.board()).unwrap();
    }

    fn get_observation(&self) -> Vec<Vec<Vec<u8>>> {
        let values_0 = IntBoardValues {
            empty: 0,
            man_1: 10,
            king_1: 20,
            man_2: 0,
            king_2: 0,
        };
        let values_1 = IntBoardValues {
            empty: 0,
            man_1: 0,
            king_1: 0,
            man_2: 10,
            king_2: 20,
        };
        vec![self.get_int_board(values_0), self.get_int_board(values_1)]
    }

    fn set_state(&mut self, state: (u8, Vec<Vec<u8>>)) -> Vec<Vec<Vec<u8>>> {
        let (current_player, board) = state;
        let mut tiles = Vec::new();
        for row in board.iter() {
            for number_tile in row.iter() {
                let tile: Box<dyn Tile> = match number_tile {
                    0 => Box::new(EmptyTile),
                    1 => Box::new(OccupiedTile::new(Box::new(ManPiece { player_id: 1 }))),
                    2 => Box::new(OccupiedTile::new(Box::new(KingPiece { player_id: 1 }))),
                    3 => Box::new(OccupiedTile::new(Box::new(ManPiece { player_id: 2 }))),
                    4 => Box::new(OccupiedTile::new(Box::new(KingPiece { player_id: 2 }))),
                    _ => unreachable!(),
                };
                tiles.push(tile)
            }
        }

        let n_rows = board.len();
        let n_columns = board[0].len();
        let mut board = Board::new(n_rows, n_columns);
        board.set_tiles(tiles);

        self.game.set_current_player(current_player as usize);
        self.game.set_board(board);
        self.game.find_available_moves();
        self.get_observation()
    }

    fn get_state(&self) -> (u8, Vec<Vec<u8>>, bool) {
        let values = IntBoardValues {
            empty: 0,
            man_1: 1,
            king_1: 2,
            man_2: 3,
            king_2: 4,
        };
        (
            self.to_play(),
            self.get_int_board(values),
            self.game.is_game_over(),
        )
    }

    fn get_int_board(&self, values: IntBoardValues) -> Vec<Vec<u8>> {
        let board = self.game.get_board();
        let mut observation: Vec<Vec<u8>> = Vec::new();
        for r in (0..board.number_rows()).rev() {
            let mut row = Vec::new();
            for c in 0..board.number_columns() {
                let tile = board.get_tile(r, c);
                let piece_str = match tile.get_piece() {
                    None => values.empty,
                    Some(piece) => match (piece.get_type(), piece.get_player_id()) {
                        (PieceType::Man, 1) => values.man_1,
                        (PieceType::King, 1) => values.king_1,
                        (PieceType::Man, 2) => values.man_2,
                        (PieceType::King, 2) => values.king_2,
                        _ => unreachable!(),
                    },
                };
                row.push(piece_str);
            }
            observation.push(row);
        }
        observation
    }
}

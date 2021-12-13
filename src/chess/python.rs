use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::{
    convert_castle_move_to_string, convert_move_to_string, convert_move_to_type,
    get_all_possible_moves, get_possible_castle_moves, is_game_over, next_state,
    player_string_to_enum, update_state, Board, Castle, Color, Move, State,
};

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pyclass]
pub struct ChessEngine {}

#[pymethods]
impl ChessEngine {
    #[new]
    fn new() -> Self {
        ChessEngine {}
    }

    fn next_state<'a>(
        &mut self,
        _py: Python<'a>,
        state_py: &'a PyDict,
        _player: &str,
        _move: &str,
    ) -> PyResult<&'a PyDict> {
        // parse state
        let state: State = convert_py_state(_py, state_py)?;

        // parse arguments
        let player: Color = player_string_to_enum(_player);

        // next state
        let move_union = convert_move_to_type(_move);
        let mut new_state = next_state(&state, player, move_union);

        // update kings under attack
        update_state(&mut new_state);
        // if both kings are checked, this position is impossible => raise exception
        if new_state.white_king_is_checked == true && new_state.black_king_is_checked == true {
            println!("Both Kings are in check: this position is impossible");
            PyException::new_err("Both Kings are in check: this position is impossible")
                .restore(_py);
        }

        // return new state
        let new_state_py = PyDict::new(_py);
        new_state.to_py_object(new_state_py);
        return Ok(new_state_py);
    }

    #[args(attack = false)]
    fn get_possible_moves<'a>(
        &mut self,
        _py: Python<'a>,
        state_py: &'a PyDict,
        _player: &str,
        attack: bool,
    ) -> PyResult<Vec<String>> {
        // parse state
        let state: State = convert_py_state(_py, state_py)?;

        // parse arguments
        let player: Color = player_string_to_enum(_player);

        let (moves, castle_moves): (Vec<Move>, Vec<Castle>) =
            get_all_possible_moves(&state, player, attack);
        // let moves: Vec<Move> = get_possible_moves(&state, player, attack);
        // let castle_moves: Vec<Castle> = get_possible_castle_moves(&state, player, attack);

        let mut moves_str: Vec<String> = moves.iter().map(|&x| convert_move_to_string(x)).collect();
        let castle_moves_str: Vec<String> = castle_moves
            .iter()
            .map(|&x| convert_castle_move_to_string(x))
            .collect();
        moves_str.extend(castle_moves_str);
        return Ok(moves_str);
    }

    fn get_castle_moves<'a>(
        &mut self,
        _py: Python<'a>,
        state_py: &'a PyDict,
        _player: &str,
    ) -> PyResult<Vec<String>> {
        // parse state
        let state: State = convert_py_state(_py, state_py)?;

        // parse arguments
        let player: Color = player_string_to_enum(_player);

        let castle_moves: Vec<Castle> = get_possible_castle_moves(&state, player, false);
        let castle_moves_str: Vec<String> = castle_moves
            .iter()
            .map(|&x| convert_castle_move_to_string(x))
            .collect();
        return Ok(castle_moves_str);
    }

    fn update_state<'a>(&mut self, _py: Python<'a>, state_py: &'a PyDict) -> PyResult<&'a PyDict> {
        // parse state
        let mut state: State = convert_py_state(_py, state_py)?;
        // update kings under attack
        update_state(&mut state);
        // Python state
        let state_py = PyDict::new(_py);
        state.to_py_object(state_py);
        return Ok(state_py);
    }

    fn is_game_over<'a>(
        &mut self,
        _py: Python<'a>,
        states_py: Vec<&'a PyDict>,
        state_py: &'a PyDict,
        _player: &str,
    ) -> PyResult<u8> {
        // parse state
        let state: State = convert_py_state(_py, state_py)?;
        // parse arguments
        let player: Color = player_string_to_enum(_player);
        let mut states: Vec<State> = Vec::new();
        for item in states_py.iter() {
            states.push(convert_py_state(_py, item)?);
        }
        // game is over
        let res = is_game_over(&states, &state, player);
        return Ok(res);
    }
}

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

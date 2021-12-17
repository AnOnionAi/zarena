#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use super::{
    checkmate, convert_castle_move_to_string, convert_move_to_string, convert_move_to_type,
    get_all_possible_moves, get_possible_castle_moves, in_stalemate, in_threefold_repetition,
    insufficient_material, next_state, player_string_to_enum, update_state, Board, Castle, Color,
    Move, State,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg(feature = "wasm")]
pub struct ChessEngine {}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg(feature = "wasm")]
impl ChessEngine {
    pub fn new() -> Self {
        ChessEngine {}
    }

    pub fn next_state(&mut self, state_js: &JsValue, _player: &str, _move: &str) -> JsValue {
        console_error_panic_hook::set_once();
        // parse arguments
        let state: State = state_js.into_serde().unwrap();

        let player: Color = player_string_to_enum(_player);

        // next state
        let move_union = convert_move_to_type(_move);
        let mut new_state = next_state(&state, player, move_union);

        // update kings under attack
        update_state(&mut new_state);
        // if both kings are checked, this position is impossible => raise exception
        if new_state.white_king_is_checked == true && new_state.black_king_is_checked == true {
            println!("Both Kings are in check: this position is impossible");
        }

        // return new state
        return JsValue::from_serde(&new_state).unwrap();
    }

    pub fn get_possible_moves(
        &mut self,
        state_js: &JsValue,
        _player: &str,
        attack: bool,
    ) -> JsValue {
        console_error_panic_hook::set_once();

        // parse arguments
        let state: State = state_js.into_serde().unwrap();

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
        return JsValue::from_serde(&moves_str).unwrap();
    }

    pub fn in_threefold_repetition(&mut self, states_js: &JsValue) -> JsValue {
        console_error_panic_hook::set_once();
        let mut states: Vec<State> = states_js.into_serde().unwrap();
        return JsValue::from_serde(&in_threefold_repetition(&states)).unwrap();
    }

    pub fn checkmate(&mut self, state_js: &JsValue, _player: &str) -> JsValue {
        console_error_panic_hook::set_once();
        let state: State = state_js.into_serde().unwrap();
        let player: Color = player_string_to_enum(_player);
        return JsValue::from_serde(&checkmate(&state, player)).unwrap();
    }

    pub fn in_stalemate(&mut self, state_js: &JsValue, _player: &str) -> JsValue {
        console_error_panic_hook::set_once();
        let state: State = state_js.into_serde().unwrap();
        let player: Color = player_string_to_enum(_player);
        return JsValue::from_serde(&in_stalemate(&state, player)).unwrap();
    }

    pub fn insufficient_material(&mut self, state_js: &JsValue) -> JsValue {
        console_error_panic_hook::set_once();
        let state: State = state_js.into_serde().unwrap();
        return JsValue::from_serde(&insufficient_material(&state.board)).unwrap();
    }

    pub fn get_castle_moves(&mut self, state_js: &JsValue, _player: &str) -> JsValue {
        // parse arguments
        let state: State = state_js.into_serde().unwrap();
        let player: Color = player_string_to_enum(_player);

        let castle_moves: Vec<Castle> = get_possible_castle_moves(&state, player, false);
        let castle_moves_str: Vec<String> = castle_moves
            .iter()
            .map(|&x| convert_castle_move_to_string(x))
            .collect();
        return JsValue::from_serde(&castle_moves_str).unwrap();
    }

    pub fn get_board(&mut self, state_js: &JsValue) -> JsValue {
        let state: State = state_js.into_serde().unwrap();
        let board = state.get_board();
        return JsValue::from_serde(&board).unwrap();
    }
}

//  - print
// [dependencies]
// web-sys = { version="0.3.5", features=[ "console" ] }

// :code
// use web_sys::console;
// let serde: JsValue = JsValue::from_serde(&count).unwrap();
// console::log_1(&serde);

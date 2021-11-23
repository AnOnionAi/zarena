use gym_chess::{Color, Move, MoveStruct, MoveUnion, State, DEFAULT_BOARD, ID_TO_ICON};
// to read input
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::num::ParseIntError; 

fn show_moves(state: &State, moves: &[Move]) {
    for (i, _move) in moves.iter().enumerate() {
        let x = _move.0 .0 as usize;
        let y = _move.0 .1 as usize;
        let board = state.get_board();
        let piece_id = board[x][y];
        let piece_icon = &ID_TO_ICON.get(&piece_id).unwrap();
        println!("{}. {} {:?} -> {:?}", i + 1, piece_icon, get_coordinates(_move.0), get_coordinates(_move.1));
    }
}

fn get_coordinates(movement: (isize, isize)) -> (char, isize) {
    (get_coordinate_x( movement.1), get_coordinate_y(movement.0))
}

fn get_coordinate_x(movement: isize) -> char {
    match movement {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => 'i'
    }
}

fn get_coordinate_y(movement: isize) -> isize{
    match movement {
        0 => 8,
        1 => 7,
        2 => 6,
        3 => 5,
        4 => 4,
        5 => 3,
        6 => 2,
        7 => 1,
        _ => 0
    }
}

fn read_input() -> Result<u32,ParseIntError> {
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let input = input.trim();
    let edad: u32 = u32::from_str(&input)?;
    Ok(edad)
}

fn main() {
    println!("====> {:?}", gym_chess::ID_TO_COLOR.get(&1));

    let king = gym_chess::ID_TO_TYPE.get(&100);
    let mut is_king = false;
    match king {
        Some(gym_chess::PieceType::King) => is_king = true,
        _ => (),
    }
    // if let gym_chess_rust::ID_TO_TYPE.get(&1) = king {
    //     println!("", );
    //     isKing = true;
    // }

    // let isKing = gym_chess_rust::PieceType::King == gym_chess_rust::ID_TO_TYPE.get(&1);
    println!("is_king ====> {:?}", is_king);

    println!("{:?}", DEFAULT_BOARD);

    let mut state = State::new(DEFAULT_BOARD, "WHITE", true, true, true, true);
    gym_chess::render_state(&state);

    let mut states: Vec<State> = Vec::new();
    gym_chess::in_threefold_repetition(&mut states);

    loop {
        // white
        {
            let player = Color::White;
            let moves = gym_chess::get_possible_moves(&state, player, false);
            // Select movement
            show_moves(&state, &moves);

            print!("Turn: white \n Please select your movement: ");
            let mut option;
            loop {
                if let Ok(e) = read_input(){
                    option = e;
                    if option <= moves.len() as u32 {
                        break;
                    }
                    println!("Enter a valid option, please");
                }else{
                    println!("Enter a valid option, please");
                }
            }
            
            let _move = moves[option as usize - 1];

            let move_struct = MoveStruct {
                is_castle: false,
                data: MoveUnion { normal_move: _move },
            };
            let new = gym_chess::next_state(&state, player, move_struct);
            state = new;
            gym_chess::render_state(&state);
            if gym_chess::insufficient_material(&state.get_board()) {
                println!("Insifficient Material");
                break;
            }
            if gym_chess::in_stalemate(&state, player) {
                println!("white in stalemate");
                break;
            }
            if gym_chess::checkmate(&state, player) {
                println!("white in checkmate");
                break;
            }
            if gym_chess::in_threefold_repetition(&mut states) {
                println!("in threefold repetition");
                break;
            }
        }
     
        // black
        {
            let player = Color::Black;
            let moves = gym_chess::get_possible_moves(&state, player, false);
            show_moves(&state, &moves);

            print!("Turn: black \n Please select your movement: ");
            let mut option;
            loop {
                if let Ok(e) = read_input(){
                    option = e;
                    if option <= moves.len() as u32 {
                        break;
                    }
                    println!("Enter a valid option, please");
                }else{
                    println!("Enter a valid option, please");
                }
            }
            
            let _move = moves[option as usize - 1];

            // next state
            let move_struct = MoveStruct {
                is_castle: false,
                data: MoveUnion { normal_move: _move },
            };
            let new = gym_chess::next_state(&state, player, move_struct);
            state = new;
            gym_chess::render_state(&state);
            if gym_chess::insufficient_material(&state.get_board()) {
                println!("Insifficient Material");
                break;
            }
            if gym_chess::in_stalemate(&state, player) {
                println!("black in stalemate");
                break;
            }
            if gym_chess::checkmate(&state, player) {
                println!("black in checkmate");
                break;
            }
            if gym_chess::in_threefold_repetition(&mut states) {
                println!("in threefold repetition");
                break;
            }
        }
    }

    // gym_chess_rust::render_state(state);
    // let player = Color::Black;
    // let moves = gym_chess_rust::get_possible_moves(&state, player, false);

    // for (i, _move) in moves.iter().enumerate() {
    //     let x = _move.0.0 as usize;
    //     let y = _move.0.1 as usize;
    //     let piece_id = state.board[x][y];
    //     let piece_icon = &ID_TO_ICON.get(&piece_id).unwrap();
    //     println!("{}. {} {:?} -> {:?}", i+1, piece_icon, _move.0, _move.1);
    // }
}

// These are not implemented yet but ideally we want the utils functions in a seperate module. 

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


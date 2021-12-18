#[cfg(feature = "python")]
pub mod python;

use rand::Rng;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TickType {
    Nought,
    Cross,
    Nil,
}

impl fmt::Display for TickType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TickType::Nought => write!(f, "O"),
            TickType::Cross => write!(f, "X"),
            TickType::Nil => write!(f, "."),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Player {
    Noughts,
    Crosses,
}

impl Player {
    #[allow(dead_code)]
    fn mark(&self) -> TickType {
        match self {
            Player::Noughts => TickType::Nought,
            Player::Crosses => TickType::Cross,
        }
    }
    #[allow(dead_code)]
    pub fn other(&self) -> Player {
        match self {
            Player::Noughts => Player::Crosses,
            Player::Crosses => Player::Noughts,
        }
    }
}

#[derive(Debug)]
pub struct Tictactoe {
    pub turn: u8,
    pub current_player: Player,
    pub board: [[TickType; 3]; 3],
    pub winner: Option<Player>,
    pub done: bool,
}

impl Tictactoe {
    #[allow(dead_code)]
    pub fn new() -> Tictactoe {
        Tictactoe {
            turn: 0,
            current_player: Player::Noughts,
            board: [[TickType::Nil; 3]; 3],
            winner: None,
            done: false,
        }
    }
    #[allow(dead_code)]
    pub fn get_state(&self) -> (u8, u8, Vec<Vec<isize>>, u8, bool) {
        (
            self.turn,
            self.to_play(),
            self.get_board_int(),
            self.get_winner_int(),
            self.done,
        )
    }
    #[allow(dead_code)]
    pub fn set_state(&mut self, state: (u8, Vec<Vec<isize>>)) -> Vec<Vec<Vec<usize>>> {
        let mut turn: u8 = 0;
        for row in state.1.iter() {
            for item in row {
                if *item != 0 {
                    turn += 1;
                }
            }
        }
        self.turn = turn;
        self.set_to_play(state.0);
        self.set_board_int(state.1);
        self.set_winner_int(2);
        self.done = false;
        self.get_observation()
    }
    #[allow(dead_code)]
    fn set_to_play(&mut self, player_int: u8) {
        self.current_player = match player_int {
            0 => Player::Crosses,
            _ => Player::Noughts,
        }
    }

    fn set_board_int(&mut self, board_int: Vec<Vec<isize>>) {
        let mut board: [[TickType; 3]; 3] = [[TickType::Nil; 3]; 3];
        for (i, row) in board_int.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                match item {
                    1 => board[i][j] = TickType::Nought,
                    -1 => board[i][j] = TickType::Cross,
                    _ => board[i][j] = TickType::Nil,
                }
            }
        }
        self.board = board;
    }

    fn set_winner_int(&mut self, player: u8) {
        self.winner = match player {
            0 => Some(Player::Crosses),
            1 => Some(Player::Noughts),
            _ => None,
        }
    }

    fn get_winner_int(&self) -> u8 {
        match self.winner {
            Some(Player::Crosses) => 0,
            Some(Player::Noughts) => 1,
            None => 2,
        }
    }

    fn get_board_int(&self) -> Vec<Vec<isize>> {
        let mut board: Vec<Vec<isize>> = vec![vec![0; 3]; 3];
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                match item {
                    TickType::Nil => board[i][j] = 0,
                    TickType::Nought => board[i][j] = 1,
                    TickType::Cross => board[i][j] = -1,
                }
            }
        }
        board
    }

    pub fn to_play(&self) -> u8 {
        match self.current_player {
            Player::Crosses => 0,
            Player::Noughts => 1,
        }
    }
    #[allow(dead_code)]
    pub fn reset(&mut self) -> Vec<Vec<Vec<usize>>> {
        self.turn = 0;
        self.current_player = Player::Crosses;
        self.board = [[TickType::Nil; 3]; 3];
        self.winner = None;
        self.get_observation()
    }
    #[allow(dead_code)]
    pub fn step(&mut self, action: usize) -> (Vec<Vec<Vec<usize>>>, f32, bool) {
        let row = action / 3;
        let col = action % 3;

        self.place_mark(row, col);
        if self.win_condition() {
            self.winner = Some(self.current_player);
        }
        self.done = self.winner.is_some() || self.legal_actions().len() == 0;
        let reward = self.get_reward();
        (self.get_observation(), reward, self.done)
    }
    #[allow(dead_code)]
    fn get_reward(&self) -> f32 {
        if self.done {
            if self.winner.is_some() {
                return 1.0;
            } else {
                return 0.5;
            }
        }
        0.0
    }

    pub fn legal_actions(&self) -> Vec<usize> {
        let mut legal_actions = Vec::new();
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TickType::Nil {
                    legal_actions.push(i * 3 + j);
                }
            }
        }
        legal_actions
    }

    fn get_observation(&self) -> Vec<Vec<Vec<usize>>> {
        let mut board_player_1 = vec![vec![0 as usize; 3]; 3];
        let mut board_player_2 = vec![vec![0 as usize; 3]; 3];
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TickType::Cross {
                    board_player_1[i][j] = 1;
                }
            }
        }
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == TickType::Nought {
                    board_player_2[i][j] = 1;
                }
            }
        }
        vec![board_player_1, board_player_2]
    }

    pub fn place_mark(&mut self, x: usize, y: usize) {
        assert_eq!(self.board[x][y], TickType::Nil);
        self.board[x][y] = self.current_player.mark();
        self.current_player = self.current_player.other();
        self.turn += 1;
    }

    pub fn win_condition(&self) -> bool {
        if self.turn < 5 {
            return false;
        }
        for i in 0..3 {
            if Tictactoe::check_all_same(&self.board[i]) {
                return true;
            }
            let temp_array = [self.board[0][i], self.board[1][i], self.board[2][i]];
            if Tictactoe::check_all_same(&temp_array) {
                return true;
            }
        }
        if Tictactoe::check_all_same(&[self.board[0][0], self.board[1][1], self.board[2][2]]) {
            return true;
        }
        if Tictactoe::check_all_same(&[self.board[0][2], self.board[1][1], self.board[2][0]]) {
            return true;
        }
        return false;
    }

    fn check_all_same(slice: &[TickType; 3]) -> bool {
        let nil_present = slice.iter().any(|&x| x == TickType::Nil);
        if nil_present {
            return false;
        }
        return slice[0] == slice[1] && slice[1] == slice[2];
    }
    #[allow(dead_code)]
    pub fn expert_action(&self) -> usize {
        let winning = self.winning_move(self.current_player);
        if winning.0 {
            return winning.1 * 3 + winning.2;
        }
        let blocking = self.winning_move(self.current_player.other());
        if blocking.0 {
            return blocking.1 * 3 + blocking.2;
        }
        if self.board[1][1] == TickType::Nil {
            return 1 * 3 + 1;
        }
        return self.random_action();
    }

    fn winning_move(&self, player: Player) -> (bool, usize, usize) {
        for (i, row) in self.board.iter().enumerate() {
            let mut winnig_move = self.have_winning_move(row, &player);
            if winnig_move.0 {
                return (winnig_move.0, i, winnig_move.1);
            }
            let column = [self.board[0][i], self.board[1][i], self.board[2][i]];
            winnig_move = self.have_winning_move(&column, &player);
            if winnig_move.0 {
                return (winnig_move.0, winnig_move.1, i);
            }
        }
        let diagonal_1 = [self.board[0][0], self.board[1][1], self.board[2][2]];
        let mut winnig_move = self.have_winning_move(&diagonal_1, &player);
        let array: [usize; 3] = [2, 1, 0];
        if winnig_move.0 {
            return (winnig_move.0, winnig_move.1, winnig_move.1);
        }
        let diagonal_2 = [self.board[0][2], self.board[1][1], self.board[2][0]];
        winnig_move = self.have_winning_move(&diagonal_2, &player);
        if winnig_move.0 {
            return (winnig_move.0, winnig_move.1, array[winnig_move.1]);
        }
        (false, 0, 0)
    }

    fn have_winning_move(&self, vector: &[TickType; 3], player: &Player) -> (bool, usize) {
        let mark_counter = self.mark_counter(vector, &player);
        if mark_counter.0 == 2 && !mark_counter.1.is_empty() {
            return (true, mark_counter.1[0]);
        }
        return (false, 0);
    }

    fn mark_counter(&self, vector: &[TickType; 3], player: &Player) -> (usize, Vec<usize>) {
        let mark = player.mark();
        let mut count = 0;
        let mut empty_spaces: Vec<usize> = Vec::new();
        for (i, item) in vector.iter().enumerate() {
            if *item == mark {
                count += 1;
            } else if *item == TickType::Nil {
                empty_spaces.push(i);
            }
        }
        (count, empty_spaces)
    }

    pub fn random_action(&self) -> usize {
        let mut rng = rand::thread_rng();
        let action = self.legal_actions()[rng.gen_range(0..self.legal_actions().len())];
        return action;
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        for row in self.board.iter() {
            let mut row_string = String::new();
            row_string.push_str(&row[0].to_string());
            row_string.push_str(" | ");
            row_string.push_str(&row[1].to_string());
            row_string.push_str(" | ");
            row_string.push_str(&row[2].to_string());
            println!("{}", row_string);
        }
    }
}

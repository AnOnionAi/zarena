pub mod python; 

use lazy_static::lazy_static;
use itertools::Itertools;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TickType {
    Cross,
    Nought,
    Nil,
}

impl fmt::Display for TickType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TickType::Cross  => write!(f, "X"),
            TickType::Nought => write!(f, "O"),
            TickType::Nil    => write!(f, "."),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Player {
    Crosses,
    Noughts,
}

impl Player {
    fn mark(&self) -> TickType {
        match self {
            Player::Crosses => TickType::Cross,
            Player::Noughts => TickType::Nought,
        }
    }

    pub fn other(&self) -> Player {
        match self {
            Player::Crosses => Player::Noughts,
            Player::Noughts => Player::Crosses,
        }
    }
}


#[derive(Debug)]
pub struct gato {
    turn: u8,
    player_turn: Player,
    pub board: [[TickType; 3]; 3],
    winner: Option<Player>,
    done: bool
}

impl gato {
    
    pub fn new() -> gato {
        gato {
            turn: 0,
            player_turn: Player::Crosses,
            board: [[TickType::Nil; 3]; 3],
            winner: None,
            done: false
        }
    }

    pub fn get_state(&self) -> (u8, u8, Vec<Vec<u8>>, u8, bool) {
        (
            self.turn,
            self.to_play(),
            self.get_board_int(),
            self.get_winner_int(),
            self.done
        )
    }

    fn get_winner_int(&self) -> u8 {
        match self.winner {
            Some(Player::Crosses) => 1,
            Some(Player::Noughts) => 2,
            None => 0
        }
    }

    fn get_board_int(&self) -> Vec<Vec<u8>> {
        let mut board: Vec<Vec<u8>> = Vec::new();
        for (i, row) in self.board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                match item {
                    TickType::Nil => board[i][j] = 0,
                    TickType::Cross => board[i][j] = 1,
                    TickType::Nought => board[i][j] = 2,
                }
            }
        } 
        board
    }

    pub fn to_play(&self) -> u8 {
        match self.player_turn {
            Player::Crosses => 0,
            Player::Noughts => 1
        }
    }

    pub fn reset(&mut self) -> Vec<Vec<Vec<usize>>> {
        self.turn = 0;
        self.player_turn = Player::Crosses;
        self.board = [[TickType::Nil; 3]; 3];
        self.winner = None;
        self.get_observation()
    }

    pub fn step(&mut self, action: usize) -> (Vec<Vec<Vec<usize>>>, usize, bool) {
        let row = action / 3;
        let col = action % 3;
        
        self.place_mark(row, col);
        self.done = self.win_condition() || self.legal_actions().len() == 0 ;
        let reward = self.get_reward();
        (self.get_observation(), reward, self.done)
    }

    fn get_reward(&self) -> usize {
        if self.done {
            return 1;
        }
        0
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
        let mut board_player_1 = vec![vec![0 as usize;3];3];
        let mut board_player_2 = vec![vec![0 as usize;3];3];
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
        let current_player = self.player_turn;
        assert_eq!(self.board[x][y], TickType::Nil);
        self.board[x][y] = current_player.other().mark();
        self.player_turn = current_player.other();
        self.turn += 1;
    }

    pub fn win_condition(&self) -> bool{
        if self.turn < 5 {
            return false;
        }
        for i in 0..3 {
            if gato::check_all_same(&self.board[i]) {
                return true;
            }
            let temp_array = [self.board[0][i], self.board[1][i], self.board[2][i]];
            if gato::check_all_same(&temp_array) {
                return true;
            }
        }
        if gato::check_all_same(&[self.board[0][0], self.board[1][1], self.board[2][2]]) {
            return true;
        }
        if gato::check_all_same(&[self.board[0][2], self.board[1][1], self.board[2][0]]) {
            return true;
        }
        return false;
    }

    fn check_all_same(slice: &[TickType;3]) -> bool {
        let nil_present = slice.iter().any(|&x| x == TickType::Nil);
        if nil_present {
            return false;
        }
        return slice[0] == slice[1] && slice[1] == slice[2];
    }
}

impl fmt::Display for gato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.board.iter() {
            let row_string = itertools::join(row, " | ");
            write!(f, "{}\n", row_string);
        }
        return write!(f, "");
    }
}

// The python library
// use pyo3::{exceptions::PyException, types::{PyTuple, PyDict}};
use pyo3::prelude::*;

use super::TwentyOne;

pub type Card = u8;
pub type Hand = Vec<Card>;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pyclass]
pub struct BlackjackEngine {
    game: TwentyOne,
}

#[pymethods]
impl BlackjackEngine {
    #[new]
    fn new(n_players: usize) -> Self {
        BlackjackEngine {
            game: TwentyOne::new(n_players),
        }
    }

    pub fn legal_actions(&self) -> PyResult<Vec<u8>> {
        // <----bet---->
        // the first step is to bet
        // 4 - 11

        // <----plays---->
        // 0 = stand
        // 1 = HIT
        // 2 = double down
        // 3 = pull apart (currently disabled)
        // example [true, true, true, false] hit, stand and doble down available

        let _legal_actions = self.game.legal_actions();
        let mut py_legal_actions = vec![];
        for i in 0..12 {
            if _legal_actions[i] {
                py_legal_actions.push(i as u8);
            }
        }
        Ok(py_legal_actions)
    }

    pub fn step(&mut self, action: u8) -> PyResult<(Vec<Vec<Vec<u8>>>, i64, bool)> {
        // a = observation
        // b = reward
        // c = done
        let (_a, b, c) = self.game.step(action, true);
        let a = array_to_vector(_a);
        Ok((a, b[1], c))
    }

    pub fn get_state(&self) -> PyResult<(Vec<Hand>, Vec<u8>, Vec<u64>, Vec<bool>, Vec<bool>, u8)> {
        let (
            players_hand,
            players_value,
            players_bet,
            players_planted,
            players_busted,
            current_player,
        ) = self.game.get_state();
        Ok((
            players_hand.clone(),
            players_value.clone(),
            players_bet.clone(),
            players_planted.clone(),
            players_busted.clone(),
            current_player,
        ))
    }

    pub fn get_total_players(&self) -> PyResult<u8> {
        Ok(self.game.get_total_players())
    }

    pub fn to_play(&self) -> PyResult<u8> {
        Ok(self.game.to_play())
    }

    pub fn reset(&mut self) -> PyResult<Vec<Vec<Vec<u8>>>> {
        let res = array_to_vector(self.game.reset());
        Ok(res)
    }
}

// conversion functions
pub fn array_to_vector(_a: [[[u8; 3]; 3]; 3]) -> Vec<Vec<Vec<u8>>> {
    let mut a = Vec::new();
    for i in 0..3 {
        let mut x = Vec::new();
        for j in 0..3 {
            let mut y = Vec::new();
            for k in 0..3 {
                y.push(_a[i][j][k]);
            }
            x.push(y);
        }
        a.push(x);
    }
    a
}

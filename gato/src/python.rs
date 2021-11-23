use pyo3::prelude::*;

use crate::{
    Tictactoe
};

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pymodule]
fn gym_tictactoe(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<TictactoeEngine>()?;

    Ok(())
}

#[pyclass]
pub struct TictactoeEngine {
    game: Tictactoe
}

#[pymethods]
impl TictactoeEngine {
    #[new]
    fn new() -> Self {
        TictactoeEngine {
            game: Tictactoe::new()
        }
    }

    pub fn legal_actions(
        &self
    ) -> PyResult<Vec<usize>> {
        let legal_actions = self.game.legal_actions();
        Ok(legal_actions)
    }

    pub fn step(
        &mut self,
        action: usize
    ) -> PyResult<(Vec<Vec<Vec<usize>>>, usize, bool)> {
        let (a, b, c) = self.game.step(action);
        Ok((a, b, c))
    }

    pub fn get_state(
        &self
    ) -> PyResult<(u8, u8, Vec<Vec<u8>>, u8, bool)> {
        let (
            turn,
            to_play,
            board_int,
            winner_int,
            done
        ) = self.game.get_state();
        Ok((
            turn,
            to_play,
            board_int,
            winner_int,
            done
        ))
    }

    pub fn to_play(&self) -> PyResult<u8> {
        Ok(self.game.to_play())
    }

    pub fn reset(&mut self) -> PyResult<Vec<Vec<Vec<usize>>>> {
        let res = self.game.reset();
        Ok(res)
    }

}
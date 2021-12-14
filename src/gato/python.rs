use super::Tictactoe;
use pyo3::prelude::*;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pyclass]
pub struct TictactoeEngine {
    game: Tictactoe,
}

#[pymethods]
impl TictactoeEngine {
    #[new]
    fn new() -> Self {
        TictactoeEngine {
            game: Tictactoe::new(),
        }
    }

    pub fn legal_actions(&self) -> PyResult<Vec<usize>> {
        let legal_actions = self.game.legal_actions();
        Ok(legal_actions)
    }

    pub fn step(&mut self, action: usize) -> PyResult<(Vec<Vec<Vec<usize>>>, f32, bool)> {
        let (a, b, c) = self.game.step(action);
        Ok((a, b, c))
    }

    pub fn get_state(&self) -> PyResult<(u8, u8, Vec<Vec<isize>>, u8, bool)> {
        let (turn, to_play, board_int, winner_int, done) = self.game.get_state();
        Ok((turn, to_play, board_int, winner_int, done))
    }

    pub fn set_state(&mut self, state: (u8, Vec<Vec<isize>>)) -> PyResult<Vec<Vec<Vec<usize>>>> {
        let observation = self.game.set_state(state);
        Ok(observation)
    }

    pub fn to_play(&self) -> PyResult<u8> {
        Ok(self.game.to_play())
    }

    pub fn reset(&mut self) -> PyResult<Vec<Vec<Vec<usize>>>> {
        let observation = self.game.reset();
        Ok(observation)
    }

    pub fn expert_action(&self) -> PyResult<usize> {
        let action = self.game.expert_action();
        Ok(action)
    }

    pub fn print(&self) {
        self.game.print();
    }
}

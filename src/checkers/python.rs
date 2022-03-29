use super::Checkers;
use pyo3::prelude::*;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pymodule]
fn checkers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<CheckersEngine>()?;

    Ok(())
}

#[pyclass]
pub struct CheckersEngine {
    game: Checkers,
}

#[pymethods]
impl CheckersEngine {
    #[new]
    fn new() -> Self {
        CheckersEngine {
            game: Checkers::new(),
        }
    }

    pub fn legal_actions(&self) -> PyResult<Vec<usize>> {
        let legal_actions = self.game.legal_actions();
        Ok(legal_actions)
    }

    pub fn step(&mut self, action: usize) -> PyResult<(Vec<Vec<Vec<u8>>>, f32, bool)> {
        let (a, b, c) = self.game.step(action);
        Ok((a, b, c))
    }

    pub fn get_state(&self) -> PyResult<(u8, Vec<Vec<u8>>, bool)> {
        let (to_play, board_int, done) = self.game.get_state();
        Ok((to_play, board_int, done))
    }

    pub fn set_state(&mut self, state: (u8, Vec<Vec<u8>>)) -> PyResult<Vec<Vec<Vec<u8>>>> {
        let observation = self.game.set_state(state);
        Ok(observation)
    }

    pub fn to_play(&self) -> PyResult<u8> {
        Ok(self.game.to_play())
    }

    pub fn reset(&mut self) -> PyResult<Vec<Vec<Vec<u8>>>> {
        let observation = self.game.reset();
        Ok(observation)
    }

    pub fn print(&self) {
        self.game.print();
    }
}

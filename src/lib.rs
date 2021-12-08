use pyo3::prelude::*;

mod gato;
mod poker;
mod blackjack;
// mod chess;

use gato::TictactoeEngine;
use poker::PokerEngine;
use blackjack::BlackjackEngine;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pymodule]
fn zarena(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<TictactoeEngine>()?;
    m.add_class::<BlackjackEngine>()?;
    m.add_class::<PokerEngine>()?;

    Ok(())
}
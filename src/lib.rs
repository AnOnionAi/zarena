use pyo3::prelude::*;

mod gato;
mod poker;
mod blackjack;
mod chess;

use gato::python::TictactoeEngine;
use poker::python::PokerEngine;
use blackjack::python::BlackjackEngine;
use chess::python::ChessEngine;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pymodule]
fn zarena(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<TictactoeEngine>()?;
    m.add_class::<BlackjackEngine>()?;
    m.add_class::<PokerEngine>()?;
    m.add_class::<ChessEngine>()?;

    Ok(())
}
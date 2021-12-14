use pyo3::prelude::*;

mod blackjack;
mod chess;
mod gato;
mod poker;

use blackjack::python::BlackjackEngine;
use chess::python::ChessEngine;
use gato::python::TictactoeEngine;
use poker::python::PokerEngine;

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

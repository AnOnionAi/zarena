#[cfg(feature = "python")]
use pyo3::prelude::*;

mod blackjack;
mod chess;
mod gato;
mod poker;

// GYMS
#[cfg(feature = "python")]
use blackjack::python::BlackjackEngine;
#[cfg(feature = "python")]
use chess::python::ChessEngine;
#[cfg(feature = "python")]
use gato::python::TictactoeEngine;
#[cfg(feature = "python")]
use poker::python::PokerEngine;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[cfg(feature = "python")]
#[pymodule]
fn zarena(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<TictactoeEngine>()?;
    m.add_class::<BlackjackEngine>()?;
    m.add_class::<PokerEngine>()?;
    m.add_class::<ChessEngine>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::poker::Poker;
    #[test]
    fn poker_works() {
        let mut poker = Poker::new(vec![100_000, 200_000], true);
        print!("{:?}", poker.reset());
        print!("{:?}", poker.step(0, true));
    }
}

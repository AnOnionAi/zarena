use pyo3::prelude::*;

// PYTHON MODULE
// ---------------------------------------------------------
// ---------------------------------------------------------
#[pymodule]
fn zarena(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<zarena>()?;

    Ok(())
}

#[pyclass]
pub struct Zarena {
    tictactoe: gato(),
}
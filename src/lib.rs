use pyo3::prelude::*;

#[pyfunction]
fn add_numbers(a: i64, b: i64) -> i64 {
    a + b
}

#[pyfunction]
fn multiply_numbers(a: f64, b: f64) -> f64 {
    a * b
}

#[pymodule]
fn minimal_rust_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(multiply_numbers, m)?)?;
    Ok(())
}
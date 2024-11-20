use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pyfunction]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[pyfunction]
fn power(a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}

#[pymodule]
fn rust_calculator(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(power, m)?)?;
    Ok(())
}


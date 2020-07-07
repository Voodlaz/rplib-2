use pyo3::prelude::*;
use pyo3::{wrap_pyfunction,create_exception};
use pyo3::exceptions::*;
use pyo3::types::*;

create_exception!(rplib, SomeError, Exception);

#[pyfunction]
fn arg(m: &PyAny) -> PyResult<&PyFloat> {
    match m.downcast::<PyFloat>() {
        Err(_) => Err({
            println!("aaaaa");
            SomeError::py_err("some_err")
        }),
        Ok(value) => Ok(value),
    }
}

fn fne(py: Python) {
    PyErr::new::<TypeError, _>("Error").restore(py)
}

#[pymodule]
fn rplib(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(arg))?;
    fne(py);
    Ok(())
}

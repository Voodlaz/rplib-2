use pyo3::prelude::*;
use pyo3::{wrap_pyfunction,create_exception};
use pyo3::exceptions::*;
use pyo3::types::*;

create_exception!(module, SomeError, Exception);

#[pyfunction]
fn arg(m: &PyAny) -> PyResult<&PyFloat> {
    match m.downcast::<PyFloat>() {
        Err(e) => Err(println!("some text")),
        Ok(value) => Ok(value),
    }
}

#[pymodule]
fn rplib(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(arg))?;
    Ok(())
}

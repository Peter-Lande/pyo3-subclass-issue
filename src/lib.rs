use pyo3::prelude::*;

#[pyclass(subclass)]
struct Base;

#[pymethods]
impl Base {
    #[new]
    fn new() -> Self {
        Base
    }

    fn super_call(&self) {
        println!("Baseclass call.");
    }
}

#[pyclass]
struct Caller {
    inner: Py<Base>
}

#[pymethods]
impl Caller {
    #[new]
    fn new(inner: Py<Base>) -> Self {
        Caller { inner }
    }

    #[getter]
    fn get_inner<'a>(&self, py: Python<'a>) -> Bound<'a, Base> {
        self.inner.clone().into_bound(py)
    }

    fn call_inner(&self) {
        Python::with_gil(|py| {
            self.inner.borrow(py).super_call();
        });
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn test_sub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Base>()?;
    m.add_class::<Caller>()?;
    Ok(())
}

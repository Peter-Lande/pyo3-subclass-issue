use pyo3::prelude::*;

#[pyclass(subclass)]
struct Base;

#[pymethods]
impl Base {
    #[new]
    fn new() -> Self {
        Base
    }

    fn first_call(&self) {
        println!("First Baseclass call.");
    }

    fn second_call(&self) {
        println!("Second Baseclass call.");
    }

    fn third_call(&self) {
        println!("Third Baseclass call.");
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

    fn call_first_inner(&self, py: Python<'_>) {
        self.inner.borrow(py).first_call();
    }

    fn call_second_inner(&self, py: Python<'_>) {
        let _ = self.inner.call_method0(py, "second_call");
    }

    fn call_third_inner(&self, py: Python<'_>) {
        let _ = self.inner.call_method0(py, "third_call");
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn test_sub(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Base>()?;
    m.add_class::<Caller>()?;
    Ok(())
}

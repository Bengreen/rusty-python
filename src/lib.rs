use pyo3::types::PyDict;
mod submodule;
use submodule::*;
mod ahoc;
use ahoc::*;
use std::time::Duration;
use tokio;


use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};


#[pyclass]
struct ExampleClass {
    #[pyo3(get, set)]
    value: i32,
}

#[pymethods]
impl ExampleClass {
    #[new]
    pub fn new(value: i32) -> Self {
        ExampleClass { value }
    }

    pub fn greetme(&self) -> &'static str {
        "Hello, world!"
    }

    pub fn personalgreet(&self, name: &str)-> PyResult<String> {
        Ok(format!("Hello there {}", name))
    }
}


#[pyfunction]
fn foo<'p>(py: Python<'p>) -> PyResult<&'p PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Python::with_gil(|py: Python| Ok(py.None()))
    })
}

#[pyfunction]
fn sleep_for<'p>(py: Python<'p>, secs: &'p PyAny) -> PyResult<&'p PyAny> {
    let secs = secs.extract()?;

    pyo3_asyncio::tokio::future_into_py(py, async move {
        tokio::time::sleep(Duration::from_secs(secs)).await;
        Python::with_gil(|py| Ok(py.None()))
    })
}

#[pyfunction]
fn rust_sleep1<'p>(py: Python<'p>) -> PyResult<&'p PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pymodule]
fn _setuptools_rust_starter(py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(foo, m)?)?;
    m.add_function(wrap_pyfunction!(sleep_for, m)?)?;
    m.add_function(wrap_pyfunction!(rust_sleep1, m)?)?;
    m.add_class::<ExampleClass>()?;
    m.add_wrapped(wrap_pymodule!(submodule))?;
    m.add_wrapped(wrap_pymodule!(ahoc))?;

    // Inserting to sys.modules allows importing submodules nicely from Python
    // e.g. from setuptools_rust_starter.submodule import SubmoduleClass
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("setuptools_rust_starter.submodule", m.getattr("submodule")?)?;
    sys_modules.set_item("setuptools_rust_starter.ahoc", m.getattr("ahoc")?)?;

    Ok(())
}

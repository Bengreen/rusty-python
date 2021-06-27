use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod submodule;
use submodule::*;

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
}

#[pymodule]
fn _setuptools_rust_starter(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ExampleClass>()?;
    m.add_wrapped(wrap_pymodule!(submodule))?;

    // Inserting to sys.modules allows importing submodules nicely from Python
    // e.g. from setuptools_rust_starter.submodule import SubmoduleClass

    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("setuptools_rust_starter.submodule", m.getattr("submodule")?)?;

    Ok(())
}
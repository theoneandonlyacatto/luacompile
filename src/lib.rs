use pyo3::prelude::*;
use pyo3::types::PyBytes;
use mlua::prelude::*;

#[pyfunction]
fn compile(source: &str, strip: bool, name: &str) -> PyResult<PyObject> {
    let lua = Lua::new();
    let chunk = lua.load(source)
        .set_name(name)
        .into_function()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
        .dump(strip);
    
    Python::with_gil(|py| {
        Ok(PyBytes::new(py, &chunk).into())
    })
}

#[pymodule]
fn luacompile(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile, m)?)?;
    Ok(())
} 
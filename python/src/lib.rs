use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
struct ExampleClass {
    #[pyo3(get)]
    num: i32,
    #[pyo3(get)]
    name: String,
}

#[pyfunction]
fn example1(input: String) -> PyResult<String> {
    println!("Hello {}!", input);
    Ok(format!("You are {}", input).to_string())
}

#[pyfunction]
fn example2(value: i32, name: String) -> Py<ExampleClass> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    Py::new(
        py,
        ExampleClass {
            num: value,
            name: name,
        },
    )
    .unwrap()
}

/// A Python module implemented in Rust.
#[allow(unused)]
#[pymodule]
fn rust_python(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(example1, m)?)?;
    m.add_function(wrap_pyfunction!(example2, m)?)?;
    m.add_class::<ExampleClass>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_example1() {
        use pyo3::types::IntoPyDict;
        let gil = Python::acquire_gil();
        let py = gil.python();
        let m = PyModule::new(py, "python_test").unwrap();
        m.add_wrapped(wrap_pyfunction!(example1)).unwrap();
        let locals = [("python_test", m)].into_py_dict(py);
        py.run(
            r#"
            assert python_test.example1("me") == "You are me"
            "#,
            None,
            Some(locals),
        )
        .unwrap();
    }
}

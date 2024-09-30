use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::{PyArray1, IntoPyArray, PyArrayMethods};
use ndarray::Array1;

#[pyfunction]
fn add_one(arr: &Bound<'_, PyArray1<f64>>, a: f64) -> Py<PyArray1<f64>> {
    // Convert PyArray1 to ndarray::Array1
    let array = unsafe { arr.as_array().to_owned() };

    
    // Increment each element by 1
    let result = array + a;
    
    // Convert ndarray::Array1 back to PyArray1
    result.into_pyarray_bound(arr.py()).unbind()
}

#[pymodule]
fn momos_aedes(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_one, m)?)?;
    Ok(())
}
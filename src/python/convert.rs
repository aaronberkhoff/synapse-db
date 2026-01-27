//! Conversion utilities between Rust and Python types.
//!
//! This module provides functions to convert between `MetaValue` and Python objects.

use crate::meta_data::MetaValue;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFloat, PyInt, PyList, PyString};
use std::collections::HashMap;

/// Convert a Python object to a `MetaValue`.
///
/// Supports conversion from:
/// - `int` -> `MetaValue::Int`
/// - `float` -> `MetaValue::Float`
/// - `str` -> `MetaValue::Str`
/// - `list` -> `MetaValue::Vec`
/// - `dict` -> `MetaValue::Map`
///
/// # Errors
///
/// Returns a `PyTypeError` if the Python object type is not supported.
pub fn py_to_metavalue(obj: &Bound<'_, PyAny>) -> PyResult<MetaValue> {
    if let Ok(val) = obj.downcast::<PyInt>() {
        Ok(MetaValue::Int(val.extract()?))
    } else if let Ok(val) = obj.downcast::<PyFloat>() {
        Ok(MetaValue::Float(val.extract()?))
    } else if let Ok(val) = obj.downcast::<PyString>() {
        Ok(MetaValue::Str(val.extract()?))
    } else if let Ok(val) = obj.downcast::<PyList>() {
        let items: PyResult<Vec<MetaValue>> =
            val.iter().map(|item| py_to_metavalue(&item)).collect();
        Ok(MetaValue::Vec(items?))
    } else if let Ok(val) = obj.downcast::<PyDict>() {
        let mut map = HashMap::new();
        for (key, value) in val.iter() {
            let key_str: String = key.extract()?;
            map.insert(key_str, py_to_metavalue(&value)?);
        }
        Ok(MetaValue::Map(map))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported type for MetaValue. Expected int, float, str, list, or dict.",
        ))
    }
}

/// Convert a `MetaValue` to a Python object.
///
/// Converts:
/// - `MetaValue::Int` -> `int`
/// - `MetaValue::Float` -> `float`
/// - `MetaValue::Str` -> `str`
/// - `MetaValue::Vec` -> `list`
/// - `MetaValue::Map` -> `dict`
pub fn metavalue_to_py(py: Python<'_>, value: &MetaValue) -> PyResult<PyObject> {
    match value {
        MetaValue::Int(v) => Ok(v.to_object(py)),
        MetaValue::Float(v) => Ok(v.to_object(py)),
        MetaValue::Str(v) => Ok(v.to_object(py)),
        MetaValue::Vec(v) => {
            let items: PyResult<Vec<PyObject>> =
                v.iter().map(|item| metavalue_to_py(py, item)).collect();
            Ok(items?.to_object(py))
        }
        MetaValue::Map(m) => {
            let dict = PyDict::new_bound(py);
            for (key, val) in m {
                dict.set_item(key, metavalue_to_py(py, val)?)?;
            }
            Ok(dict.into())
        }
    }
}

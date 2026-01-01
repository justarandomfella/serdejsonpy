use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBytes, PyDict, PyList};
use pythonize::{depythonize, pythonize};
use serde_json::{Map, Value};

#[pyclass]
struct SerdeJSON;

#[pymethods]
impl SerdeJSON {
    #[new]
    fn new() -> Self {
        SerdeJSON
    }

    // ==================== Getter / Setter ====================

    /// Get value at a key from a JSON object
    fn get(&self, py: Python, obj: &Bound<'_, PyAny>, key: &str) -> PyResult<Py<PyAny>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        if let Value::Object(map) = val {
            let result = map.get(key).cloned().unwrap_or(Value::Null);
            pythonize(py, &result)
                .map(|bound| bound.unbind())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
        } else {
            Ok(py.None())
        }
    }

    /// Set value at a key in a JSON object
    fn set(&self, obj: &Bound<'_, PyAny>, key: &str, value: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let mut val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let new_value: Value = depythonize(value)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        if let Value::Object(map) = &mut val {
            map.insert(key.to_string(), new_value);
            pythonize(obj.py(), &val)
                .map(|bound| bound.unbind())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Target is not a JSON object"))
        }
    }
    
    /// Convert Python object to compact JSON string
    fn to_json(&self, obj: &Bound<'_, PyAny>) -> PyResult<String> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize: {}", e)))?;
        serde_json::to_string(&val)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON error: {}", e)))
    }

    /// Convert Python object to pretty-printed JSON string
    fn to_json_pretty(&self, obj: &Bound<'_, PyAny>) -> PyResult<String> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize: {}", e)))?;
        serde_json::to_string_pretty(&val)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON error: {}", e)))
    }

    /// Convert Python object to compact JSON bytes
    fn to_bytes(&self, py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyBytes>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize: {}", e)))?;
        let bytes = serde_json::to_vec(&val)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON error: {}", e)))?;
        Ok(PyBytes::new_bound(py, &bytes).unbind())
    }

    /// Convert Python object to pretty-printed JSON bytes
    fn to_bytes_pretty(&self, py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyBytes>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize: {}", e)))?;
        let bytes = serde_json::to_vec_pretty(&val)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON error: {}", e)))?;
        Ok(PyBytes::new_bound(py, &bytes).unbind())
    }

    // ==================== Deserialization Methods ====================
    
    /// Parse JSON string to Python object
    fn from_json(&self, py: Python, json_str: &str) -> PyResult<Py<PyAny>> {
        let val: Value = serde_json::from_str(json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e)))?;
        pythonize(py, &val)
            .map(|bound| bound.unbind())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e)))
    }

    /// Parse JSON bytes to Python object
    fn from_bytes(&self, py: Python, b: &Bound<'_, PyBytes>) -> PyResult<Py<PyAny>> {
        let val: Value = serde_json::from_slice(b.as_bytes())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON bytes: {}", e)))?;
        pythonize(py, &val)
            .map(|bound| bound.unbind())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e)))
    }

    // ==================== Value Type Checking ====================
    
    /// Check if JSON value is null
    fn is_null(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_null())
    }

    /// Check if JSON value is a boolean
    fn is_boolean(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_boolean())
    }

    /// Check if JSON value is a number
    fn is_number(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_number())
    }

    /// Check if JSON value is a string
    fn is_string(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_string())
    }

    /// Check if JSON value is an array
    fn is_array(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_array())
    }

    /// Check if JSON value is an object
    fn is_object(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_object())
    }

    /// Check if JSON value is an integer (i64)
    fn is_i64(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_i64())
    }

    /// Check if JSON value is an unsigned integer (u64)
    fn is_u64(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_u64())
    }

    /// Check if JSON value is a floating point number (f64)
    fn is_f64(&self, obj: &Bound<'_, PyAny>) -> PyResult<bool> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.is_f64())
    }

    // ==================== Value Extraction ====================
    
    /// Extract value as boolean
    fn as_bool(&self, obj: &Bound<'_, PyAny>) -> PyResult<Option<bool>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.as_bool())
    }

    /// Extract value as i64
    fn as_i64(&self, obj: &Bound<'_, PyAny>) -> PyResult<Option<i64>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.as_i64())
    }

    /// Extract value as u64
    fn as_u64(&self, obj: &Bound<'_, PyAny>) -> PyResult<Option<u64>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.as_u64())
    }

    /// Extract value as f64
    fn as_f64(&self, obj: &Bound<'_, PyAny>) -> PyResult<Option<f64>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.as_f64())
    }

    /// Extract value as string
    fn as_str(&self, obj: &Bound<'_, PyAny>) -> PyResult<Option<String>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(val.as_str().map(|s| s.to_string()))
    }

    // ==================== Advanced Operations ====================
    
    /// Deep merge two JSON objects
    fn merge(&self, py: Python, a: &Bound<'_, PyAny>, b: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let mut va: Value = depythonize(a)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let vb: Value = depythonize(b)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        fn merge_values(a: &mut Value, b: &Value) {
            match (a, b) {
                (Value::Object(map_a), Value::Object(map_b)) => {
                    for (k, v) in map_b {
                        match map_a.get_mut(k) {
                            Some(existing) => merge_values(existing, v),
                            None => { map_a.insert(k.clone(), v.clone()); }
                        }
                    }
                }
                (a_slot, b_val) => *a_slot = b_val.clone(),
            }
        }

        merge_values(&mut va, &vb);
        pythonize(py, &va)
            .map(|bound| bound.unbind())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e)))
    }

    /// Get value at path (list of keys/indices)
    fn get_path(&self, py: Python, obj: &Bound<'_, PyAny>, path: Vec<String>) -> PyResult<Py<PyAny>> {
        let mut val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        for key in &path {
            val = match &val {
                Value::Object(map) => map.get(key).cloned().unwrap_or(Value::Null),
                Value::Array(arr) => {
                    if let Ok(idx) = key.parse::<usize>() {
                        arr.get(idx).cloned().unwrap_or(Value::Null)
                    } else {
                        Value::Null
                    }
                }
                _ => Value::Null,
            };
        }
        
        pythonize(py, &val)
            .map(|bound| bound.unbind())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e)))
    }

    /// Access nested value using JSON Pointer (RFC 6901)
    fn pointer(&self, py: Python, obj: &Bound<'_, PyAny>, pointer: &str) -> PyResult<Py<PyAny>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        match val.pointer(pointer) {
            Some(result) => pythonize(py, result)
                .map(|bound| bound.unbind())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e))),
            None => Ok(py.None()),
        }
    }

    /// Flatten nested JSON object with dot notation keys
    fn flatten(&self, py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyDict>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        let dict = PyDict::new_bound(py);
        
        fn flatten_rec(prefix: String, val: &Value, dict: &Bound<'_, PyDict>, py: Python) -> PyResult<()> {
            match val {
                Value::Object(map) => {
                    for (k, v) in map {
                        let new_key = if prefix.is_empty() {
                            k.clone()
                        } else {
                            format!("{}.{}", prefix, k)
                        };
                        flatten_rec(new_key, v, dict, py)?;
                    }
                }
                Value::Array(arr) => {
                    for (i, v) in arr.iter().enumerate() {
                        let new_key = format!("{}[{}]", prefix, i);
                        flatten_rec(new_key, v, dict, py)?;
                    }
                }
                other => {
                    let py_val = pythonize(py, other)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                    dict.set_item(prefix, py_val)?;
                }
            }
            Ok(())
        }
        
        flatten_rec(String::new(), &val, &dict, py)?;
        Ok(dict.unbind())
    }

    /// Get all keys from JSON object (recursive)
    #[pyo3(signature = (obj, recursive=None))]
    fn keys(&self, py: Python, obj: &Bound<'_, PyAny>, recursive: Option<bool>) -> PyResult<Py<PyList>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        let recursive = recursive.unwrap_or(false);
        let list = PyList::empty_bound(py);
        
        fn collect_keys(val: &Value, list: &Bound<'_, PyList>, recursive: bool, py: Python) -> PyResult<()> {
            if let Value::Object(map) = val {
                for key in map.keys() {
                    list.append(key)?;
                    if recursive {
                        if let Some(nested) = map.get(key) {
                            collect_keys(nested, list, recursive, py)?;
                        }
                    }
                }
            }
            Ok(())
        }
        
        collect_keys(&val, &list, recursive, py)?;
        Ok(list.unbind())
    }

    /// Get all values from JSON object/array (recursive)
    #[pyo3(signature = (obj, recursive=None))]
    fn values(&self, py: Python, obj: &Bound<'_, PyAny>, recursive: Option<bool>) -> PyResult<Py<PyList>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        let recursive = recursive.unwrap_or(false);
        let list = PyList::empty_bound(py);
        
        fn collect_values(val: &Value, list: &Bound<'_, PyList>, recursive: bool, py: Python) -> PyResult<()> {
            match val {
                Value::Object(map) => {
                    for v in map.values() {
                        let py_val = pythonize(py, v)
                            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                        list.append(py_val)?;
                        if recursive {
                            collect_values(v, list, recursive, py)?;
                        }
                    }
                }
                Value::Array(arr) => {
                    for v in arr {
                        let py_val = pythonize(py, v)
                            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                        list.append(py_val)?;
                        if recursive {
                            collect_values(v, list, recursive, py)?;
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        }
        
        collect_values(&val, &list, recursive, py)?;
        Ok(list.unbind())
    }

    /// Count total number of values in nested structure
    fn count_values(&self, obj: &Bound<'_, PyAny>) -> PyResult<usize> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        fn count_rec(val: &Value) -> usize {
            match val {
                Value::Object(map) => 1 + map.values().map(count_rec).sum::<usize>(),
                Value::Array(arr) => 1 + arr.iter().map(count_rec).sum::<usize>(),
                _ => 1,
            }
        }
        
        Ok(count_rec(&val))
    }

    /// Find all paths to a specific value
    fn find_paths(&self, py: Python, obj: &Bound<'_, PyAny>, target: &Bound<'_, PyAny>) -> PyResult<Py<PyList>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let target_val: Value = depythonize(target)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        let list = PyList::empty_bound(py);
        
        fn find_rec(current: &Value, target: &Value, path: Vec<String>, results: &Bound<'_, PyList>, py: Python) -> PyResult<()> {
            if current == target {
                let path_list = PyList::empty_bound(py);
                for p in &path {
                    path_list.append(p)?;
                }
                results.append(path_list)?;
            }
            
            match current {
                Value::Object(map) => {
                    for (k, v) in map {
                        let mut new_path = path.clone();
                        new_path.push(k.clone());
                        find_rec(v, target, new_path, results, py)?;
                    }
                }
                Value::Array(arr) => {
                    for (i, v) in arr.iter().enumerate() {
                        let mut new_path = path.clone();
                        new_path.push(i.to_string());
                        find_rec(v, target, new_path, results, py)?;
                    }
                }
                _ => {}
            }
            Ok(())
        }
        
        find_rec(&val, &target_val, Vec::new(), &list, py)?;
        Ok(list.unbind())
    }

    /// Remove null values recursively
    fn remove_nulls(&self, py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        fn remove_nulls_rec(val: Value) -> Value {
            match val {
                Value::Object(map) => {
                    let filtered: Map<String, Value> = map
                        .into_iter()
                        .filter(|(_, v)| !v.is_null())
                        .map(|(k, v)| (k, remove_nulls_rec(v)))
                        .collect();
                    Value::Object(filtered)
                }
                Value::Array(arr) => {
                    let filtered: Vec<Value> = arr
                        .into_iter()
                        .filter(|v| !v.is_null())
                        .map(remove_nulls_rec)
                        .collect();
                    Value::Array(filtered)
                }
                other => other,
            }
        }
        
        let cleaned = remove_nulls_rec(val);
        pythonize(py, &cleaned)
            .map(|bound| bound.unbind())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e)))
    }

    /// Sort all object keys alphabetically (recursive)
    fn sort_keys(&self, py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let mut val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        fn sort_rec(val: &mut Value) {
            match val {
                Value::Object(map) => {
                    let mut entries: Vec<_> = map.iter_mut().collect();
                    entries.sort_by(|a, b| a.0.cmp(b.0));
                    for (_, v) in entries {
                        sort_rec(v);
                    }
                }
                Value::Array(arr) => {
                    for v in arr {
                        sort_rec(v);
                    }
                }
                _ => {}
            }
        }
        
        sort_rec(&mut val);
        pythonize(py, &val)
            .map(|bound| bound.unbind())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Pythonize error: {}", e)))
    }

    /// Get depth of nested structure
    fn depth(&self, obj: &Bound<'_, PyAny>) -> PyResult<usize> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        
        fn depth_rec(val: &Value) -> usize {
            match val {
                Value::Object(map) => {
                    1 + map.values().map(depth_rec).max().unwrap_or(0)
                }
                Value::Array(arr) => {
                    1 + arr.iter().map(depth_rec).max().unwrap_or(0)
                }
                _ => 0,
            }
        }
        
        Ok(depth_rec(&val))
    }

    /// Validate JSON string without parsing to Python
    fn validate(&self, json_str: &str) -> PyResult<bool> {
        match serde_json::from_str::<Value>(json_str) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Minify JSON string (remove whitespace)
    fn minify(&self, json_str: &str) -> PyResult<String> {
        let val: Value = serde_json::from_str(json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e)))?;
        serde_json::to_string(&val)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON error: {}", e)))
    }

    /// Compare two JSON objects for structural equality
    fn equals(&self, a: &Bound<'_, PyAny>, b: &Bound<'_, PyAny>) -> PyResult<bool> {
        let va: Value = depythonize(a)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let vb: Value = depythonize(b)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(va == vb)
    }

    /// Get size in bytes of JSON representation
    fn size(&self, obj: &Bound<'_, PyAny>) -> PyResult<usize> {
        let val: Value = depythonize(obj)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let bytes = serde_json::to_vec(&val)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("JSON error: {}", e)))?;
        Ok(bytes.len())
    }
}

#[pymodule]
fn serdejsonpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SerdeJSON>()?;
    Ok(())
}

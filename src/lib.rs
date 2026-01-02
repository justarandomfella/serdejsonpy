use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};
use pyo3::exceptions::PyValueError;
use pythonize::{depythonize, pythonize};
use serde_json::Value;

#[pyclass]
pub struct SerdeJSON;

#[pymethods]
impl SerdeJSON {
    #[new]
    #[inline(always)]
    fn new() -> Self { SerdeJSON }

    #[inline(always)]
    fn get<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, key: &str) -> PyResult<Bound<'py, PyAny>> {
        match depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))? {
            Value::Object(map) => pythonize(py, map.get(key).unwrap_or(&Value::Null))
                .map_err(|e| PyValueError::new_err(e.to_string())),
            _ => Ok(py.None().into_bound(py))
        }
    }

    #[inline(always)]
    fn set<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, key: &Bound<'py, PyAny>, value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let mut val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let new_value = depythonize::<Value>(value).map_err(|e| PyValueError::new_err(e.to_string()))?;
        match &mut val {
            Value::Object(map) => { 
                map.insert(key.extract::<String>()?, new_value); 
                pythonize(py, &val).map_err(|e| PyValueError::new_err(e.to_string()))
            },
            _ => Err(pyo3::exceptions::PyTypeError::new_err("Target is not a JSON object"))
        }
    }

    #[inline(always)]
    fn to_json(&self, obj: &Bound<PyAny>) -> PyResult<String> { 
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        serde_json::to_string(&val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn to_json_pretty(&self, obj: &Bound<PyAny>) -> PyResult<String> { 
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        serde_json::to_string_pretty(&val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn to_bytes<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let bytes = serde_json::to_vec(&val).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyBytes::new(py, &bytes))
    }

    #[inline(always)]
    fn to_bytes_pretty<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let bytes = serde_json::to_vec_pretty(&val).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyBytes::new(py, &bytes))
    }

    #[inline(always)]
    fn from_json<'py>(&self, py: Python<'py>, json_str: &str) -> PyResult<Bound<'py, PyAny>> {
        let val = serde_json::from_str::<Value>(json_str).map_err(|e| PyValueError::new_err(e.to_string()))?;
        pythonize(py, &val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn from_bytes<'py>(&self, py: Python<'py>, b: &Bound<'py, PyBytes>) -> PyResult<Bound<'py, PyAny>> {
        let val: Value = serde_json::from_slice(b.as_bytes()).map_err(|e| PyValueError::new_err(e.to_string()))?;
        pythonize(py, &val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn is_null(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_null()) 
    }

    #[inline(always)]
    fn is_boolean(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_boolean()) 
    }

    #[inline(always)]
    fn is_number(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_number()) 
    }

    #[inline(always)]
    fn is_string(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_string()) 
    }

    #[inline(always)]
    fn is_array(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_array()) 
    }

    #[inline(always)]
    fn is_object(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_object()) 
    }

    #[inline(always)]
    fn is_i64(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_i64()) 
    }

    #[inline(always)]
    fn is_u64(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_u64()) 
    }

    #[inline(always)]
    fn is_f64(&self, obj: &Bound<PyAny>) -> PyResult<bool> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.is_f64()) 
    }

    #[inline(always)]
    fn as_bool(&self, obj: &Bound<PyAny>) -> PyResult<Option<bool>> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.as_bool()) 
    }

    #[inline(always)]
    fn as_i64(&self, obj: &Bound<PyAny>) -> PyResult<Option<i64>> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.as_i64()) 
    }

    #[inline(always)]
    fn as_u64(&self, obj: &Bound<PyAny>) -> PyResult<Option<u64>> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.as_u64()) 
    }

    #[inline(always)]
    fn as_f64(&self, obj: &Bound<PyAny>) -> PyResult<Option<f64>> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.as_f64()) 
    }

    #[inline(always)]
    fn as_str(&self, obj: &Bound<PyAny>) -> PyResult<Option<String>> { 
        Ok(depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?.as_str().map(|s| s.to_string())) 
    }

    #[inline(always)]
    fn merge<'py>(&self, py: Python<'py>, a: &Bound<'py, PyAny>, b: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        fn merge_values(a: &mut Value, b: &Value) {
            match (a, b) {
                (Value::Object(a_map), Value::Object(b_map)) => {
                    for (k, v) in b_map {
                        a_map.entry(k.clone())
                            .and_modify(|x| merge_values(x, v))
                            .or_insert_with(|| v.clone());
                    }
                },
                (a_slot, b_val) => *a_slot = b_val.clone(),
            }
        }
        let mut va = depythonize::<Value>(a).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let vb = depythonize::<Value>(b).map_err(|e| PyValueError::new_err(e.to_string()))?;
        merge_values(&mut va, &vb);
        pythonize(py, &va).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn get_path<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, path: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        let mut val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        for key in path {
            val = match val {
                Value::Object(map) => map.get(&key).cloned().unwrap_or(Value::Null),
                Value::Array(arr) => key.parse::<usize>().ok().and_then(|i| arr.get(i).cloned()).unwrap_or(Value::Null),
                _ => Value::Null,
            }
        }
        pythonize(py, &val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn pointer<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, pointer: &str) -> PyResult<Bound<'py, PyAny>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        match val.pointer(pointer) {
            Some(res) => pythonize(py, res).map_err(|e| PyValueError::new_err(e.to_string())),
            None => Ok(py.None().into_bound(py))
        }
    }

    #[inline(always)]
    fn flatten<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let dict = PyDict::new(py);

        fn rec<'py>(val: &Value, prefix: &str, dict: &Bound<'py, PyDict>, py: Python<'py>) -> PyResult<()> {
            match val {
                Value::Object(map) => {
                    for (k, v) in map {
                        let new_prefix = if prefix.is_empty() { k.clone() } else { format!("{}.{}", prefix, k) };
                        rec(v, &new_prefix, dict, py)?;
                    }
                },
                Value::Array(arr) => {
                    for (i, v) in arr.iter().enumerate() {
                        rec(v, &format!("{}[{}]", prefix, i), dict, py)?;
                    }
                },
                _ => dict.set_item(prefix, pythonize(py, val).map_err(|e| PyValueError::new_err(e.to_string()))?)?,
            }
            Ok(())
        }

        rec(&val, "", &dict, py)?;
        Ok(dict)
    }

    #[inline(always)]
    #[pyo3(signature = (obj, recursive=None))]
    fn keys<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, recursive: Option<bool>) -> PyResult<Bound<'py, PyList>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let list = PyList::empty(py);

        fn collect<'py>(val: &Value, list: &Bound<'py, PyList>, recursive: bool, py: Python<'py>) -> PyResult<()> {
            if let Value::Object(map) = val {
                for k in map.keys() {
                    list.append(k)?;
                    if recursive { collect(&map[k], list, true, py)?; }
                }
            }
            Ok(())
        }

        collect(&val, &list, recursive.unwrap_or(false), py)?;
        Ok(list)
    }

    #[inline(always)]
    #[pyo3(signature = (obj, recursive=None))]
    fn values<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, recursive: Option<bool>) -> PyResult<Bound<'py, PyList>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let list = PyList::empty(py);

        fn collect<'py>(val: &Value, list: &Bound<'py, PyList>, recursive: bool, py: Python<'py>) -> PyResult<()> {
            match val {
                Value::Object(map) => {
                    for v in map.values() {
                        list.append(pythonize(py, v)?)?;
                        if recursive { collect(v, list, true, py)?; }
                    }
                }
                Value::Array(arr) => {
                    for v in arr {
                        list.append(pythonize(py, v)?)?;
                        if recursive { collect(v, list, true, py)?; }
                    }
                }
                _ => {}
            }
            Ok(())
        }

        collect(&val, &list, recursive.unwrap_or(false), py)?;
        Ok(list)
    }

    #[inline(always)]
    fn count_values(&self, obj: &Bound<PyAny>) -> PyResult<usize> {
        fn rec(val: &Value) -> usize {
            match val {
                Value::Object(m) => 1 + m.values().map(rec).sum::<usize>(),
                Value::Array(a) => 1 + a.iter().map(rec).sum::<usize>(),
                _ => 1
            }
        }
        Ok(rec(&depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?))
    }

    #[inline(always)]
    fn find_paths<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>, target: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let target_val = depythonize::<Value>(target).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let paths = PyList::empty(py);

        fn rec<'py>(val: &Value, target: &Value, current: &mut Vec<String>, results: &Bound<'py, PyList>, py: Python<'py>) -> PyResult<()> {
            if val == target {
                results.append(current.clone())?;
            }
            match val {
                Value::Object(map) => {
                    for (k, v) in map {
                        current.push(k.clone());
                        rec(v, target, current, results, py)?;
                        current.pop();
                    }
                },
                Value::Array(arr) => {
                    for (i, v) in arr.iter().enumerate() {
                        current.push(i.to_string());
                        rec(v, target, current, results, py)?;
                        current.pop();
                    }
                },
                _ => {}
            }
            Ok(())
        }

        rec(&val, &target_val, &mut Vec::new(), &paths, py)?;
        Ok(paths)
    }

    #[inline(always)]
    fn remove_nulls<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        fn rec(val: Value) -> Value {
            match val {
                Value::Object(m) => Value::Object(m.into_iter()
                    .filter(|(_, v)| !v.is_null())
                    .map(|(k, v)| (k, rec(v)))
                    .collect()),
                Value::Array(a) => Value::Array(a.into_iter()
                    .filter(|v| !v.is_null())
                    .map(rec)
                    .collect()),
                o => o
            }
        }
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        pythonize(py, &rec(val)).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn sort_keys<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        fn rec(val: &mut Value) {
            match val {
                Value::Object(m) => {
                    let mut entries: Vec<_> = m.iter_mut().collect();
                    entries.sort_by(|a, b| a.0.cmp(b.0));
                    for (_, v) in entries { rec(v); }
                },
                Value::Array(a) => { for v in a { rec(v); } },
                _ => {}
            }
        }
        let mut val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        rec(&mut val);
        pythonize(py, &val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn depth(&self, obj: &Bound<PyAny>) -> PyResult<usize> {
        fn rec(val: &Value) -> usize {
            match val {
                Value::Object(m) => 1 + m.values().map(rec).max().unwrap_or(0),
                Value::Array(a) => 1 + a.iter().map(rec).max().unwrap_or(0),
                _ => 0
            }
        }
        Ok(rec(&depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?))
    }

    #[inline(always)]
    fn validate(&self, json_str: &str) -> PyResult<bool> { Ok(serde_json::from_str::<Value>(json_str).is_ok()) }

    #[inline(always)]
    fn minify(&self, json_str: &str) -> PyResult<String> {
        let val = serde_json::from_str::<Value>(json_str).map_err(|e| PyValueError::new_err(e.to_string()))?;
        serde_json::to_string(&val).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[inline(always)]
    fn equals(&self, a: &Bound<PyAny>, b: &Bound<PyAny>) -> PyResult<bool> {
        let va = depythonize::<Value>(a).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let vb = depythonize::<Value>(b).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(va == vb)
    }

    #[inline(always)]
    fn size(&self, obj: &Bound<PyAny>) -> PyResult<usize> {
        let val = depythonize::<Value>(obj).map_err(|e| PyValueError::new_err(e.to_string()))?;
        serde_json::to_vec(&val).map(|v| v.len()).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pymodule]
fn serdejsonpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SerdeJSON>()?;
    Ok(())
}

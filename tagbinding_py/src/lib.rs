#[pyo3::pymodule]
mod rs_tags {
    use tagcore;
    use pyo3::{Bound, PyResult, create_exception, prelude::*, pyclass, types::PyType};

    create_exception!(rs_tags, PyTagError, pyo3::exceptions::PyException);

    #[pyclass]
    struct Tag {
        inner: tagcore::Tag,
    }

    #[pymethods]
    impl Tag {
        #[classmethod]
        pub fn simple(_class: Bound<PyType>, value: String) -> Self {
            Tag { inner: tagcore::Tag::Simple(value) }
        }

        #[classmethod]
        pub fn kv(_class: Bound<PyType>, key: String, value: String) -> Self {
            Tag { inner: tagcore::Tag::KV(key, value) }
        }

        #[getter]
        pub fn kind(&self) -> &str {
            match self.inner {
                tagcore::Tag::Simple(_) => "Simple",
                tagcore::Tag::KV(_, _) => "KV",
            }
        }

        pub fn is_simple(&self) -> bool {
            matches!(self.inner, tagcore::Tag::Simple(_))
        }

        pub fn is_kv(&self) -> bool {
            matches!(self.inner, tagcore::Tag::KV(_, _))
        }

        #[getter]
        pub fn simple_value(&self) -> Option<String> {
            match &self.inner {
                tagcore::Tag::Simple(v) => Some(v.clone()),
                _ => None,
            }
        }

        #[getter]
        pub fn kv_key(&self) -> Option<String> {
            match &self.inner {
                tagcore::Tag::KV(k, _) => Some(k.clone()),
                _ => None,
            }
        }

        #[getter]
        pub fn kv_value(&self) -> Option<String> {
            match &self.inner {
                tagcore::Tag::KV(_, v) => Some(v.clone()),
                _ => None,
            }
        }


        fn __repr__(&self) -> PyResult<String> {
            Ok(match &self.inner {
                tagcore::Tag::Simple(v) => format!("Tag.Simple({:?})", v),
                tagcore::Tag::KV(k, v) => format!("Tag.KV({:?}, {:?})", k, v),
            })
        }
    }

    impl From<tagcore::Tag> for Tag {
        fn from(tag: tagcore::Tag) -> Self {
            Tag { inner: tag }
        }
    }

    #[pyclass]
    struct TagWorkspace {
        inner: tagcore::Workspace,
    }

    #[pymethods]
    impl TagWorkspace {
        #[classmethod]
        pub fn open_workspace(_class: Bound<PyType>, directory: std::path::PathBuf, name: String) -> PyResult<Self> {
            match tagcore::Workspace::open_workspace(directory, &name) {
                Ok(x) => Ok(TagWorkspace {inner: x}),
                Err(err) => Err(PyTagError::new_err(err.to_string())),
            }
        }

        #[classmethod]
        pub fn create_workspace(_class: Bound<PyType>, directory: std::path::PathBuf, name: String) -> PyResult<Self> {
            match tagcore::Workspace::create_workspace(directory, &name) {
                Ok(x) => Ok(TagWorkspace { inner: x }),
                Err(err) => Err(PyTagError::new_err(err.to_string())),
            }
        }

        pub fn scan_for_tagfiles(&mut self) {
            self.inner.scan_for_tagfiles();
        }

        pub fn add_tag_to_file(&mut self, path_to_file: std::path::PathBuf, tag_1: String, tag_2: Option<String>) -> PyResult<()> {
            self.inner.add_tag_to_file(path_to_file, tag_1, tag_2).map_err(|e| PyTagError::new_err(e.to_string()))
        }

        pub fn remove_tag_from_file(&mut self, path_to_file: std::path::PathBuf, tag_1: String, tag_2: Option<String>) -> PyResult<()> {
            self.inner.remove_tag_from_file(path_to_file, tag_1, tag_2).map_err(|e| PyTagError::new_err(e.to_string()))
        }

        pub fn get_tags_for_file_name(&self, full_path_to_file: std::path::PathBuf) -> PyResult<Vec<Tag>> {
            let rval = self.inner.get_tags_for_file_name(full_path_to_file).map_err(|e| PyTagError::new_err(e.to_string()))?;
            Ok(rval.into_iter().map(Tag::from).collect())
        }

        pub fn get_name(&self) -> &str {
            self.inner.get_name()
        }

        pub fn query_exact(&self, text: &str, simple: bool, key: bool, value: bool) -> std::collections::HashMap<String, Vec<Tag>> {
            let result = self.inner.query_exact(text, simple, key, value);
            let mut rv: std::collections::HashMap<String, Vec<Tag>> = std::collections::HashMap::new();
            for (fname, vector) in result {
                let v = vector.into_iter().map(|item| {
                    Tag { inner: item }
                }).collect();

                rv.insert(fname, v);
            };
            rv
        }

        pub fn query_fuzzy(&self, text: &str, simple: bool, key: bool, value: bool) -> std::collections::HashMap<String, Vec<Tag>> {
            let result = self.inner.query_fuzzy(text, simple, key, value);
            let mut rv: std::collections::HashMap<String, Vec<Tag>> = std::collections::HashMap::new();
            for (fname, vector) in result {
                let v = vector.into_iter().map(|item| {
                    Tag { inner: item }
                }).collect();

                rv.insert(fname, v);
            };
            rv
        }
    }
}


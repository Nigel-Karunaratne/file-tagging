#[pyo3::pymodule]
mod rs_tags {
    use tagcore;
    use pyo3::{PyResult, create_exception, prelude::*, pyclass, types::PyType};

    create_exception!(tags, PyTagError, pyo3::exceptions::PyException);

    #[pyclass]
    struct TagWorkspace {
        inner: tagcore::Workspace,
    }

    #[pymethods]
    impl TagWorkspace {
        #[classmethod]
        fn open_or_create_workspace(_class: Bound<PyType>, directory: std::path::PathBuf, name: String) -> PyResult<Self> {
            let x = tagcore::Workspace::open_or_create_workspace(directory, name);
            match x {
                Ok(x) => Ok(TagWorkspace {inner: x}),
                Err(err) => Err(PyTagError::new_err(err.to_string())),
            }
        }

        fn scan_for_tagfiles(&mut self) {
            self.inner.scan_for_tagfiles();
        }

        fn add_tag_to_file(&mut self, path_to_file: std::path::PathBuf, tag_1: String, tag_2: Option<String>) -> PyResult<()> {
            self.inner.add_tag_to_file(path_to_file, tag_1, tag_2).map_err(|e| PyTagError::new_err(e.to_string()))
        }

        fn remove_tag_from_file(&mut self, path_to_file: std::path::PathBuf, tag_1: String, tag_2: Option<String>) -> PyResult<()> {
            self.inner.remove_tag_from_file(path_to_file, tag_1, tag_2).map_err(|e| PyTagError::new_err(e.to_string()))
        }
    }
}


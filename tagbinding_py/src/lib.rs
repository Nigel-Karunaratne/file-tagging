#[pyo3::pymodule]
mod tags {
    use tagcore;
    use pyo3::prelude::*;

    #[pyfunction]
    fn triple(a: u64, b: u64, c: u64) -> u64 {
        tagcore::three_mult(a,b,c)
    }
}


extern crate rustmetamap as rmm;

use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::exceptions;

#[pyclass]
pub struct Model {
    pub Model: Container<dyn rmm::MetamapModel>,
}

#[pyclass(extends=Model)]
pub struct Metamap {}

#[pymethods] {
    #[new]
    #[args(kwargs = "*")]

    pub fn new(kwargs: Option<&PyDict>) -> PyResult<(Self, Trainer)> {
        let mut builder = rmm::Metamap::new();
    }
}
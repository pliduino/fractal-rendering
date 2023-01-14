use pyo3::prelude::*;

#[derive(Clone, Copy)]
#[pyclass]
pub enum Generators {
    Mandelbrot,
    Cubic,
    Cosz,
}

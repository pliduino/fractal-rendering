use pyo3::prelude::*;

mod color;
mod fractal_generator;

#[pymodule]
fn rust_generation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<fractal_generator::FractalGenerator>()?;
    m.add_class::<fractal_generator::generators::Generators>()?;
    Ok(())
}

use pyo3::prelude::*;

#[derive(Clone, Copy)]
#[pyclass]
pub enum Generators {
    Mandelbrot,
    Cubic,
    Cosz,
}

pub fn gen_mandelbrot(z: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
    z * z
}
pub fn gen_cubic(z: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
    z * z * z
}
pub fn gen_cosz(z: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
    z.cos() * z
}

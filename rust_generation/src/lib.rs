mod color;
use color::Color;

use num_complex::{self, ComplexFloat};

use pyo3::prelude::*;

#[pyclass]
struct MandelbrotGenerator {}

#[pymethods]
impl MandelbrotGenerator {
    unsafe fn generate_mandelbrot(
        &mut self,
        img_size: usize,
        iterations: u32,
        offset: [f64; 2],
        step: f64,
        escape_constant: f64,
    ) -> PyResult<Vec<f64>> {
        let mut texture_data = vec![0.0; img_size * img_size * 4];

        let color_1 = color::Color {
            r: 200.0,
            g: 25.0,
            b: 25.0,
        };
        let color_2 = Color {
            r: 0.0,
            g: 200.0,
            b: 255.0,
        };

        for i in 0..(img_size * img_size) {
            let rgb = {
                let x = ((i % img_size) as f64 - (img_size as f64 / 2.0)) * step + offset[0];
                let y = (f64::floor(i as f64 / img_size as f64) - (img_size as f64 / 2.0)) * step
                    + offset[1];

                let escape_time = match calc_mandelbrot(x, y, iterations, escape_constant) {
                    Ok(x) => x,
                    Err(_) => panic!("Error!"),
                };

                let color: Color;

                if escape_time > iterations / 2 {
                    let factor = ((iterations) - (escape_time)) as f64 / (iterations) as f64;
                    let factor = factor * 2.0;

                    color = color_1 * factor;
                } else {
                    let factor = ((iterations) - (escape_time)) as f64 / (iterations) as f64;
                    let factor = (factor - 0.5) * 2.0;
                    color = color_1 + ((color_2 - color_1) * factor);
                }

                color
            };

            texture_data[(i * 4) as usize] = rgb.r / 255.0;
            texture_data[((i * 4) + 1) as usize] = rgb.g / 255.0;
            texture_data[((i * 4) + 2) as usize] = rgb.b / 255.0;
            texture_data[((i * 4) + 3) as usize] = 1.0;
        }

        Ok(texture_data)
    }

    #[new]
    fn py_new(_size: usize) -> MandelbrotGenerator {
        MandelbrotGenerator {}
    }
}

unsafe fn calc_mandelbrot(
    x: f64,
    y: f64,
    iterations: u32,
    escape_constant: f64,
) -> Result<u32, u8> {
    let constant = num_complex::Complex::new(x, y);
    let mut next_z = num_complex::Complex::new(x, y);

    for i in 0..iterations {
        next_z = next_z * next_z;

        let distance = next_z.abs();

        if distance > escape_constant {
            return Ok(i);
        }

        next_z += constant;
    }

    Ok(iterations)
}

#[pymodule]
fn rust_generation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MandelbrotGenerator>()?;
    Ok(())
}

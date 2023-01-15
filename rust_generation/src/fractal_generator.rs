use num_complex::{self, ComplexFloat};
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

mod generators;
mod thread_handle;

use crate::color::Color;
pub use generators::Generators;
use generators::*;
pub use thread_handle::ThreadHandle;

#[pyclass]
pub struct FractalGenerator {}

#[pymethods]
impl FractalGenerator {
    unsafe fn generate_fractal(
        &mut self,
        img_size: usize,
        iterations: u32,
        offset: [f64; 2],
        step: f64,
        escape_constant: f64,
        gen_func: Generators,
    ) -> PyResult<Vec<f64>> {
        let texture_data = vec![0.0; img_size * img_size * 4];

        let color_1 = Color {
            r: 200.0,
            g: 25.0,
            b: 25.0,
        };
        let color_2 = Color {
            r: 0.0,
            g: 200.0,
            b: 255.0,
        };

        let mut queue = VecDeque::<usize>::new();

        const THREAD_COUNT: usize = 4;

        for i in 0..(img_size * img_size) {
            queue.push_back(i);
        }

        let message = ThreadHandle {
            queue: Arc::new(Mutex::new(queue)),
            texture_data: Arc::new(Mutex::new(texture_data)),
            img_size,
            escape_constant,
            color_1,
            color_2,
            step,
            gen_func,
            offset,
            iterations,
        };

        unsafe {
            let threads: Vec<_> = (0..THREAD_COUNT)
                .map(|_i| {
                    let message_reference = message.clone();
                    thread::spawn(move || {
                        let message = message_reference;
                        loop {
                            let i = match message.queue.lock().unwrap().pop_front() {
                                Some(x) => x,
                                None => break,
                            };

                            let rgb = {
                                let x = ((i % message.img_size) as f64
                                    - (message.img_size as f64 / 2.0))
                                    * message.step
                                    + message.offset[0];
                                let y = (f64::floor(i as f64 / message.img_size as f64)
                                    - (message.img_size as f64 / 2.0))
                                    * step
                                    + offset[1];

                                let gen_func = match message.gen_func {
                                    Generators::Mandelbrot => gen_mandelbrot,
                                    Generators::Cubic => gen_cubic,
                                    Generators::Cosz => gen_cosz,
                                };

                                let escape_time = match calc_fractal(
                                    x,
                                    y,
                                    message.iterations,
                                    message.escape_constant,
                                    gen_func,
                                ) {
                                    Ok(x) => x,
                                    Err(_) => panic!("Error!"),
                                };

                                let color: Color;

                                if escape_time > message.iterations / 2 {
                                    let factor = ((message.iterations) - (escape_time)) as f64
                                        / (message.iterations) as f64;
                                    let factor = factor * 2.0;

                                    color = color_1 * factor;
                                } else {
                                    let factor = ((message.iterations) - (escape_time)) as f64
                                        / (message.iterations) as f64;
                                    let factor = (factor - 0.5) * 2.0;
                                    color = message.color_1
                                        + ((message.color_2 - message.color_1) * factor);
                                }

                                color
                            };

                            message.texture_data.lock().unwrap()[(i * 4) as usize] = rgb.r / 255.0;
                            message.texture_data.lock().unwrap()[((i * 4) + 1) as usize] =
                                rgb.g / 255.0;
                            message.texture_data.lock().unwrap()[((i * 4) + 2) as usize] =
                                rgb.b / 255.0;
                            message.texture_data.lock().unwrap()[((i * 4) + 3) as usize] = 1.0;
                        }
                    })
                })
                .collect();

            for handle in threads {
                handle.join().unwrap();
            }
        }

        Ok(Arc::try_unwrap(message.texture_data)
            .unwrap()
            .into_inner()
            .unwrap())
    }

    #[new]
    fn py_new(_size: usize) -> FractalGenerator {
        FractalGenerator {}
    }
}

unsafe fn calc_fractal(
    x: f64,
    y: f64,
    iterations: u32,
    escape_constant: f64,
    gen_func: fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
) -> Result<u32, u8> {
    let constant = num_complex::Complex::new(x, y);
    let mut next_z = num_complex::Complex::new(x, y);

    for i in 0..iterations {
        next_z = gen_func(next_z);

        let distance = next_z.abs();

        if distance > escape_constant {
            return Ok(i);
        }

        next_z += constant;
    }

    Ok(iterations)
}

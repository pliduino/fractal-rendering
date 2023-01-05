use pyo3::prelude::*;

#[pyfunction]
unsafe fn calc_mandelbrot(x: f64, y: f64, iterations: u32, escape_constant: f64) -> PyResult<u32> {
    let constant = [x, y];
    let mut next_z = [x, y];

    for i in 0..iterations {
        // Z squared
        next_z[0] = (next_z[0] * next_z[0]) - (next_z[1] * next_z[1]);
        next_z[1] = 2.0 * next_z[0] * next_z[1];

        let distance = f64::sqrt((next_z[0] * next_z[0]) + (next_z[1] * next_z[1]));

        if distance > escape_constant {
            return Ok(i);
        }

        next_z[0] += constant[0];
        next_z[1] += constant[1];
    }

    Ok(iterations)
}

#[pyfunction]
unsafe fn generate_mandelbrot(
    img_size: usize,
    iterations: u32,
    offset: [f64; 2],
    step: f64,
    escape_constant: f64,
) -> PyResult<Vec<f64>> {
    let mut texture_data = vec![0.0; img_size * img_size * 4];

    for i in (0..(img_size * img_size)).step_by(4) {
        let x = ((i % img_size) as f64 - (img_size as f64 / 2.0)) * step + offset[0];
        let y =
            (f64::floor(i as f64 / img_size as f64) - (img_size as f64 / 2.0)) * step + offset[1];

        let escape_time = match calc_mandelbrot(x, y, iterations, escape_constant) {
            Ok(x) => x,
            Err(_) => panic!("Error!"),
        };

        let mut rgb = [0.0; 3];

        if escape_time > iterations / 2 {
            let factor = ((iterations) - (escape_time)) as f64 / (iterations) as f64;
            let factor = factor * 2.0;

            rgb[0] = factor * (200.0 / 255.0);
            rgb[1] = factor * (25.0 / 255.0);
            rgb[2] = factor * (25.0 / 255.0);
        } else {
            let factor = ((iterations) - (escape_time)) as f64 / (iterations) as f64;
            let factor = (factor - 0.5) * 2.0;

            rgb[0] = 200.0 / 255.0;
            rgb[1] = 25.0 / 255.0;
            rgb[2] = 25.0 / 255.0;

            rgb[0] += factor * (-200.0 / 255.0);
            rgb[1] += factor * (75.0 / 255.0);
            rgb[2] += factor * (230.0 / 255.0);
        }
        texture_data[i as usize] = rgb[0];
        texture_data[(i + 1) as usize] = rgb[1];
        texture_data[(i + 2) as usize] = rgb[2];
        texture_data[(i + 3) as usize] = 1.0;
    }

    Ok(texture_data)
}

#[pymodule]
fn rust_generation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_mandelbrot, m)?)?;
    Ok(())
}

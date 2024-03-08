use crate::color::Color;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Shares data between FractalGenerator threads
pub struct ThreadHandle {
    pub queue: Arc<Mutex<VecDeque<usize>>>,
    pub texture_data: Arc<Mutex<Vec<f64>>>,
    pub color_1: Color,
    pub img_size: usize,
    pub escape_constant: f64,
    pub step: f64,
    pub gen_func: fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
    pub offset: [f64; 2],
    pub iterations: u32,
}

impl Clone for ThreadHandle {
    fn clone(&self) -> Self {
        ThreadHandle {
            queue: self.queue.clone(),
            texture_data: self.texture_data.clone(),
            color_1: self.color_1,
            img_size: self.img_size,
            escape_constant: self.escape_constant,
            step: self.step,
            gen_func: self.gen_func,
            offset: self.offset,
            iterations: self.iterations,
        }
    }
}

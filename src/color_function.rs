pub mod color_domain;
pub mod color_space;
pub mod escape_time;

use num_complex::Complex;

pub fn mandelbrot_grayscale(c: Complex<f32>, iteration_max: usize) -> [u8; 3] {
    escape_time::mandelbrot(c, iteration_max)
        .map(|t| [(t % u8::MAX as usize) as u8; 3])
        .unwrap_or([0; _])
}

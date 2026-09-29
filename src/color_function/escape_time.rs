use core::iter::{Sum, repeat_n};

use bon::builder;

use num_complex::Complex;
use num_traits::{Num, NumCast};

#[builder]
pub fn escape_time<N>(
    initial_value: Complex<N>,
    mut iterative_equation: impl FnMut(Complex<N>) -> Complex<N>,
    mut escape_condition: impl FnMut(Complex<N>) -> bool,
    iteration_max: usize,
) -> Option<usize>
where
    N: Num + Copy,
{
    let mut z = initial_value;
    for n in 0..iteration_max {
        z = iterative_equation(z);
        if escape_condition(z) {
            return Some(n);
        }
    }
    return None;
}

pub fn mandelbrot<
    N, //
>(
    c: Complex<N>,
    iteration_max: usize,
) -> Option<usize>
where
    N: Num //
        + NumCast
        + PartialOrd
        + Copy
        + Sum
        + Clone,
{
    let four = repeat_n(N::one(), 4).sum();
    escape_time()
        .initial_value(Complex {
            re: N::zero(),
            im: N::zero(),
        })
        .iteration_max(iteration_max)
        .iterative_equation(|z| z * z + c)
        .escape_condition(|z| z.norm_sqr() > four)
        .call()
}

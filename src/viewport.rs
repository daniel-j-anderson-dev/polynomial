use crate::complex_extension::*;

use core::{fmt::Display, ops::Neg};

use rayon::prelude::*;

use bon::{bon, builder};

use num_complex::Complex;
use num_traits::{AsPrimitive, Num, NumCast};

#[derive(Debug, Clone, Copy)]
pub struct Viewport<N> {
    pub top_left: Complex<N>,
    pub dimensions: Complex<N>,
    pub pixel_row_count: u32,
    pub pixel_column_count: u32,
    pub delta_pixel: Complex<N>,
}
impl<N> Viewport<N>
where
    N: Num + Neg<Output = N> + NumCast + Copy,
{
    pub fn new(top: N, left: N, width: N, height: N, image_height: u32, image_width: u32) -> Self {
        let viewport_dimensions = Complex::new(width, height);
        let image_dimensions = Complex::new(image_width, image_height);
        let image_dimensions_n = image_dimensions.map(N::from).map(Option::unwrap);
        Self {
            top_left: Complex { re: left, im: top },
            dimensions: viewport_dimensions,
            delta_pixel: Complex {
                re: width / image_dimensions_n.re,
                im: -(height / image_dimensions_n.im),
            },
            pixel_row_count: image_height,
            pixel_column_count: image_width,
        }
    }
    pub fn center(&self) -> Complex<N> {
        let two = N::one() + N::one();
        Complex {
            re: self.top_left.re + (self.dimensions.re / two),
            im: self.top_left.im - (self.dimensions.im / two),
        }
    }
}
#[bon]
impl<N> Viewport<N>
where
    N: Num + Neg<Output = N> + NumCast + Copy,
{
    #[builder]
    pub fn builder(
        center: Complex<N>,
        width: N,
        height: N,
        image_height: u32,
        image_width: u32,
    ) -> Self {
        let two = N::one() + N::one();
        let left = center.re - width / two;
        let top = center.im + height / two;
        Self::new(top, left, width, height, image_height, image_width)
    }
}
impl<N> Viewport<N>
where
    N: Num + NumCast + Clone,
{
    pub fn pixel_to_complex(&self, row_index: u32, column_index: u32) -> Complex<N> {
        let pixel_index = Complex {
            re: column_index,
            im: row_index,
        }
        .map(N::from)
        .map(Option::unwrap);
        let delta = pixel_index.elementwise_multiply(self.delta_pixel.clone());
        return self.top_left.clone() + delta;
    }
}
impl<N> Viewport<N>
where
    N: Num + NumCast + Clone + Sync + Send,
{
    pub fn pixel_indexes(&self) -> impl ParallelIterator<Item = (u32, u32)> + '_ {
        (0..self.pixel_row_count)
            .into_par_iter()
            .flat_map(move |row_index| {
                (0..self.pixel_column_count)
                    .into_par_iter()
                    .map(move |column_index| (row_index, column_index))
            })
    }

    pub fn pixels(&self) -> impl ParallelIterator<Item = Complex<N>> + '_ {
        self.pixel_indexes()
            .map(|(row, col)| self.pixel_to_complex(row, col))
    }
}
impl<N> core::fmt::Display for Viewport<N>
where
    N: Display + Num + PartialEq + PartialOrd + Clone + Copy + Neg<Output = N> + NumCast,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "resolution_{}x{};center_{};dimensions_{};",
            self.pixel_column_count,
            self.pixel_row_count,
            self.center(),
            self.dimensions
        )
    }
}

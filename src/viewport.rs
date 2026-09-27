use crate::complex_extension::*;

use core::{fmt::Display, ops::Neg};

use rayon::prelude::*;

use bon::{bon, builder};

use num_complex::Complex;
use num_traits::{Num, NumCast};

#[derive(Debug, Clone, Copy)]
pub struct Viewport<N> {
    pub top_left: Complex<N>,
    pub dimensions: Complex<N>,
    pub image_dimensions: Complex<u32>,
    pub delta_pixel: Complex<N>,
}
impl<N> Viewport<N>
where
    N: Num + Neg<Output = N> + NumCast + Copy,
{
    pub fn new(
        top: N,
        left: N,
        width: N,
        height: N,
        image_height: u32,
        image_width: u32,
    ) -> Self {
        let viewport_dimensions = Complex::new(width, height);
        let image_dimensions = Complex::new(image_width, image_height);
        let image_dimensions_n = image_dimensions.map(N::from).map(Option::unwrap);
        Self {
            top_left: Complex { re: top, im: left },
            delta_pixel: viewport_dimensions.elementwise_divide(image_dimensions_n),
            image_dimensions: image_dimensions,
            dimensions: viewport_dimensions,
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
        top: N,
        left: N,
        width: N,
        height: N,
        image_height: u32,
        image_width: u32,
    ) -> Self {
        Self::new(top, left, width, height, image_height, image_width)
    }
}
impl<N> Viewport<N>
where
    N: Num + NumCast + Copy + Sync + Send,
{
    pub fn pixel_to_complex(&self, pixel_index: Complex<u32>) -> Complex<N> {
        let pixel_index = pixel_index.map(N::from).map(Option::unwrap);
        let delta = pixel_index.elementwise_multiply(self.delta_pixel);
        return self.top_left + delta;
    }

    pub fn pixel_indexes(&self) -> impl ParallelIterator<Item = Complex<u32>> + '_ {
        (0..self.image_dimensions.re)
            .into_par_iter()
            .flat_map(move |row_index| {
                (0..self.image_dimensions.im)
                    .into_par_iter()
                    .map(move |column_index| Complex {
                        re: column_index,
                        im: row_index,
                    })
            })
    }

    pub fn pixels(&self) -> impl ParallelIterator<Item = Complex<N>> + '_ {
        self.pixel_indexes()
            .map(|pixel_index| self.pixel_to_complex(pixel_index))
    }

    pub fn pixels_enumerated(
        &self,
    ) -> impl ParallelIterator<Item = (Complex<u32>, Complex<N>)> + '_ {
        self.pixel_indexes()
            .map(|pixel_index| (pixel_index, self.pixel_to_complex(pixel_index)))
    }
}
impl<N> core::fmt::Display for Viewport<N>
where
    N: Display + Num + PartialEq + PartialOrd + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "image_dimensions_{};top_left_{};dimensions_{};",
            self.image_dimensions, self.top_left, self.dimensions
        )
    }
}

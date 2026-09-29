use crate::complex_extension::*;

use core::{fmt::Display, ops::Neg};

use bon::{bon, builder};

use num_complex::Complex;
use num_traits::{Num, NumCast};

#[derive(Debug, Clone, Copy)]
pub struct Viewport<N> {
    pub bottom_left: Complex<N>,
    pub dimensions: Complex<N>,
    pub pixel_row_count: u32,
    pub pixel_column_count: u32,
    pub delta_pixel: Complex<N>,
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
        let dimensions = Complex {
            re: width,
            im: height,
        };
        let bottom_left = center - dimensions / two;
        let image_dimensions = Complex {
            re: N::from(image_width).unwrap(),
            im: N::from(image_height).unwrap(),
        };
        let delta_pixel = dimensions.elementwise_divide(image_dimensions);
        Self {
            bottom_left,
            dimensions,
            pixel_row_count: image_height,
            pixel_column_count: image_width,
            delta_pixel,
        }
    }
}

impl<N> Viewport<N>
where
    N: Num + Clone,
{
    pub fn center(&self) -> Complex<N> {
        let two = N::one() + N::one();
        self.bottom_left.clone() + self.dimensions.clone() / two
    }
}
impl<N> Viewport<N>
where
    N: Num + NumCast + Clone,
{
    pub fn pixel_to_complex(&self, row_index: u32, column_index: u32) -> Complex<N> {
        let pixel_index = Complex {
            re: N::from(column_index).unwrap(),
            im: N::from(self.pixel_row_count - 1 - row_index).unwrap(),
        };
        let delta = pixel_index.elementwise_multiply(self.delta_pixel.clone());
        return self.bottom_left.clone() + delta;
    }
}

impl<N> Display for Viewport<N>
where
    N: Display + Num + PartialEq + PartialOrd + Clone + Copy + Neg<Output = N> + NumCast,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

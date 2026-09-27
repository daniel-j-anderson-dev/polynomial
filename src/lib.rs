pub mod complex_extension;
pub mod viewport;
pub mod color_function;

use crate::{complex_extension::*, viewport::Viewport};

use image::ImageBuffer;
use num_complex::Complex;
use num_traits::{FromPrimitive, Num};
use rayon::prelude::*;

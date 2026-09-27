use crate::Viewport;

use core::iter::{Sum, repeat_n};

use bon::builder;

use image::ImageBuffer;
use num_complex::Complex;
use num_traits::{AsPrimitive, Float, Num, NumCast};

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

pub fn mandelbrot_escape_time<
    N, //
>(
    c: Complex<N>,
    iteration_max: usize,
) -> Option<usize>
where
    N: Num + NumCast + Copy + PartialOrd + Clone + Sum,
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

#[test]
fn _x2_plus_x1_plus_1x0() {
    let image_width = 500;
    let image_height = image_width;
    let height = 200.0f32;
    let width = height;
    let Complex { re: left, im: top } = Complex {
        re: -width,
        im: height,
    } / 2.0;
    let viewport = Viewport::builder() //
        .image_width(image_width)
        .image_height(image_height)
        .height(height)
        .width(width)
        .top(top)
        .left(left)
        .build();

    fn f(x: Complex<f32>) -> Complex<f32> {
        x * x + x + 1.0
    }

    fn complex_to_color(c: Complex<f32>) -> image::Rgb<u8> {
        let angle = c.arg();
        let norm = c.norm();

        let hue = RgbHue::from_radians(angle);
        let brightness = ((norm.ln() / 5.0) + 0.5).clamp(0.0, 1.0);

        use core::marker::PhantomData;
        use palette::{Hsv, IntoColor, RgbHue, Srgb};
        let hsv_to_srgb = <Hsv as IntoColor<Srgb>>::into_color;
        let color = Hsv {
            hue,
            value: brightness,
            saturation: 1.0,
            standard: PhantomData,
        };
        let color = hsv_to_srgb(color);
        let color = [color.red, color.green, color.blue];
        let color = color.map(|channel| (channel * (u8::MAX as f32)) as u8);
        color.into()
    }

    let generated_image = ImageBuffer::from_par_fn(
        viewport.image_dimensions.re,
        viewport.image_dimensions.im,
        |row_index, column_index| {
            let c = viewport.pixel_to_complex(Complex {
                re: column_index,
                im: row_index,
            });
            let fc = f(c);
            let color = complex_to_color(fc);
            color
        },
    );
    let now = jiff::Zoned::now().strftime("%Y-%m-%d-%H-%M-%S").to_string();
    let path = format!("./out/{viewport}_{now}.png");
    println!("generated {}", path);
    generated_image.save(path).unwrap();
}

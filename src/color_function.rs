mod escape_time;

use num_complex::Complex;

fn hsv_to_rgb(hue_radians: f32, saturation: f32, value: f32) -> [u8; 3] {
    use core::f32::consts::{FRAC_PI_3, TAU};

    // normalize params
    let h = hue_radians.rem_euclid(TAU);
    let s = saturation.clamp(0.0, 1.0);
    let v = value.clamp(0.0, 1.0);

    let c = v * s;
    let h_prime = h / FRAC_PI_3;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = v - c;

    const fn f32_percent_to_u8(f: f32) -> u8 {
        (f * u8::MAX as f32).round() as u8
    }

    if h_prime < 1.0 {
        [c, x, 0.0]
    } else if h_prime < 2.0 {
        [x, c, 0.0]
    } else if h_prime < 3.0 {
        [0.0, c, x]
    } else if h_prime < 4.0 {
        [0.0, x, c]
    } else if h_prime < 5.0 {
        [x, 0.0, c]
    } else {
        [c, 0.0, x]
    }
    .map(|c| c + m)
    .map(f32_percent_to_u8)
}

fn color_domain(c: Complex<f32>) -> [u8; 3] {
    let (r, theta) = c.to_polar();
    hsv_to_rgb(theta, r, 1.0)
}

fn mandelbrot_grayscale(c: Complex<f32>, iteration_max: usize) -> [u8; 3] {
    match escape_time::mandelbrot(c, iteration_max) {
        Some(t) => [(t % 255) as _; 3],
        None => [0; 3],
    }
}

#[test]
fn _x2_plus_x1_plus_1x0() {
    let image_width = 1000;
    let image_height = image_width;
    let height = 4.0f32;
    let width = height;
    let center = Complex::ZERO;
    let viewport = crate::Viewport::builder() //
        .image_width(image_width)
        .image_height(image_height)
        .height(height)
        .width(width)
        .center(center)
        .build();
    
    fn f(z: Complex<f32>) -> Complex<f32> {
        (11.0 * z.powu(3)) - z
    }
    let generated_image = {
        use image::{ImageBuffer, Rgb};
        ImageBuffer::<Rgb<_>, _>::from_par_fn(
            viewport.pixel_column_count,
            viewport.pixel_row_count,
            |row_index, column_index| {
                let z = viewport.pixel_to_complex(row_index, column_index);
                color_domain(f(z)).into()
                // mandelbrot_grayscale(z, 1000).into()
            },
        )
    };
    let now = jiff::Zoned::now().strftime("%Y-%m-%d-%H-%M-%S").to_string();
    let path = format!("./out/mandelbrot_{viewport}_{now}.png");
    generated_image.save(&path).unwrap();
    println!("generated {}", path);
}

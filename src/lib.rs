pub mod color_function;
pub mod complex_extension;
pub mod viewport;

pub use crate::{color_function::*, viewport::Viewport};

#[test]
fn generate_image() {
    use crate::complex_extension::*;
    use core::f32::consts::PI;
    use num_complex::Complex;

    let image_width = 1000;
    let image_height = image_width;
    let height = 6.0f32;
    let width = height;
    // let center = Complex::ZERO;
    let center = Complex::new(0.051395, 0.0);
    let viewport = Viewport::builder() //
        .image_width(image_width)
        .image_height(image_height)
        .height(height)
        .width(width)
        .center(center)
        .build();

    fn f(z: Complex<f32>) -> Complex<f32> {
        z.powu(3) + z.powu(2) - z - 2.0
    }

    fn df_dz(z: Complex<f32>) -> Complex<f32> {
        3.0 * z.powu(2) + 2.0 * z - 1.0
    }

    const BLACK: [u8; 3] = [0; _];
    const GRAY: [u8; 3] = [u8::MAX / 2; _];

    let generated_image = {
        use image::{ImageBuffer, Rgb};
        ImageBuffer::<Rgb<_>, _>::from_par_fn(
            viewport.pixel_column_count,
            viewport.pixel_row_count,
            |column_index, row_index| {
                // calculate the transformation for this pixel
                let z = viewport.pixel_to_complex(row_index, column_index);
                // let f = core::convert::identity;
                let fz = f(z);
                let df = df_dz(z);
                let dfz_norm = df.norm(); // amount output space grew/shrank

                // output color black if this pixel is a solution
                let approximate_distance = fz.norm() / (dfz_norm + 1e-6);
                let solution_threshold = 0.02;
                let is_z_solution = (approximate_distance) <= solution_threshold;

                // output color gray if this pixel was an input space grid line
                let closest_grid_line = fz.map(f32::round);
                let distance_to_grid_line = (closest_grid_line - fz).map(f32::abs);
                let distance_to_grid_line = distance_to_grid_line.map(|x| x / dfz_norm);
                let grid_line_threshold = 0.005; // Fixed spatial thickness in complex plane coordinates
                let is_fz_input_grid_line = distance_to_grid_line.re <= grid_line_threshold
                    || distance_to_grid_line.im <= grid_line_threshold;

                let color = if is_z_solution {
                    BLACK
                } else if is_fz_input_grid_line {
                    GRAY
                } else {
                    color_domain::hsv_angle_norm_1(fz)
                };
                color.into()
            },
        )
    };

    let now = jiff::Zoned::now().strftime("%Y-%m-%d-%H-%M-%S").to_string();
    let prefix = "./out";
    let path = format!("{prefix}/poly_{viewport}_{now}.png");

    std::fs::create_dir_all(prefix).unwrap();
    generated_image.save(&path).unwrap();

    println!("generated {}", path);
}

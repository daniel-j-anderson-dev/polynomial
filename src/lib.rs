pub mod color_function;
pub mod complex_extension;
pub mod viewport;

pub use crate::{color_function::*, viewport::Viewport};

#[test]
fn generate_image() {
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

    let generated_image = {
        use image::{ImageBuffer, Rgb};
        ImageBuffer::<Rgb<_>, _>::from_par_fn(
            viewport.pixel_column_count,
            viewport.pixel_row_count,
            |column_index, row_index| {
                let z = viewport.pixel_to_complex(row_index, column_index);
                let fz = f(z);

                let approximate_distance = fz.norm() / (df_dz(z).norm() + 1e-6);
                let solution_threshold = 0.02;
                let is_z_solution = (approximate_distance) <= solution_threshold;

                let color = if is_z_solution {
                    [0; 3]
                } else {
                    color_domain::hsv_angle_norm_1(fz)
                };
                color.into()
            },
        )
    };

    let now = jiff::Zoned::now().strftime("%Y-%m-%d-%H-%M-%S").to_string();
    let prefix = "./out";
    let path = format!("{prefix}/(x^3)+(x^2)-x-2_{viewport}_{now}.png");

    std::fs::create_dir_all(prefix).unwrap();
    generated_image.save(&path).unwrap();

    println!("generated {}", path);
}

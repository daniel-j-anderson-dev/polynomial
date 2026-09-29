use num_complex::Complex;

pub fn hsv_angle_norm_1(c: Complex<f32>) -> [u8; 3] {
    let (r, theta) = c.to_polar();
    crate::color_space::hsv_to_rgb(theta, r, 1.0)
}

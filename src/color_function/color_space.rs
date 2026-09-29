use core::f32::consts::{FRAC_PI_3, TAU};

const fn f32_percent_to_u8(f: f32) -> u8 {
    (f * u8::MAX as f32).round() as u8
}

pub fn hsv_to_rgb(hue_radians: f32, saturation: f32, value: f32) -> [u8; 3] {
    // normalize params
    let h = hue_radians.rem_euclid(TAU);
    let s = saturation.clamp(0.0, 1.0);
    let v = value.clamp(0.0, 1.0);

    let c = v * s;
    let h_prime = h / FRAC_PI_3;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = v - c;

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

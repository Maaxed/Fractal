use spirv_std::glam::DVec2;
use crate::complex::Complex;

const iteration_count: u32 = 100;

pub fn mandelbrot_value(c: DVec2) -> f32
{
    let mut z = DVec2::ZERO;

    for i in 0..iteration_count
    {
        let length_squared = z.length_squared();
        if length_squared > 4.0
        {
            return i as f32 / iteration_count as f32;
        }
        z = z.comp_squared() + c;
    }

    return 1.0;
}
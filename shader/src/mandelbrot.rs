use spirv_std::glam::DVec2;
use crate::complex::Complex;

const ITERATION_COUNT: u32 = 512;

pub fn mandelbrot_value(c: DVec2) -> f32
{
    let mut z = DVec2::ZERO;

    for i in 0..ITERATION_COUNT
    {
        let length_squared = z.length_squared();
        if length_squared > 4.0
        {
            return i as f32 / ITERATION_COUNT as f32;
        }
        z = z.comp_squared() + c;
    }

    1.0
}
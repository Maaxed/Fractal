use spirv_std::glam::Vec2;
use crate::complex::Complex;

const ITERATION_COUNT: u32 = 255;

pub fn mandelbrot_value(c: Vec2) -> u32
{
    let mut z = Vec2::ZERO;

    for i in 0..ITERATION_COUNT
    {
        let length_squared = z.length_squared();
        if length_squared > 4.0
        {
            return i;
        }
        z = z.comp_squared() + c;
    }

    ITERATION_COUNT
}
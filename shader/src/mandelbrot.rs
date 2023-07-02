use spirv_std::glam::DVec2;
use crate::complex::Complex;

const ITERATION_COUNT: u32 = 1024;

pub fn mandelbrot_value(pos: DVec2) -> f32
{
    mandelbrot_base(DVec2::ZERO, pos)
}

pub fn mandelbrot_julia_set(pos: DVec2, secondary_pos: DVec2) -> f32
{
    mandelbrot_base(pos, secondary_pos)
}

pub fn mandelbrot_base(mut z: DVec2, c: DVec2) -> f32
{
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

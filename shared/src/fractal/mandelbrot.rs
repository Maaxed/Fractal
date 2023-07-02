use glam::{DVec2, dvec2};
use crate::complex::Complex;
use super::escape_time_method::compute_escape_time;

const ITERATION_COUNT: u32 = 1024;

pub fn mandelbrot_value(pos: DVec2) -> f32
{
    // Cardioid / bulb checking
    let q = (pos + dvec2(-0.25, 0.0)).length_squared();

    if q * (q + (pos.x - 0.25)) <= 0.25 * pos.y * pos.y // the point is within the cardioid
        || (pos + dvec2(1.0, 0.0)).length_squared() < 0.25 * 0.25 // the poitn is within the period-2 bulb
    {
        return 1.0
    }

    mandelbrot_base(DVec2::ZERO, pos)
}

pub fn mandelbrot_julia_set(pos: DVec2, secondary_pos: DVec2) -> f32
{
    mandelbrot_base(pos, secondary_pos)
}

fn mandelbrot_base(z: DVec2, c: DVec2) -> f32
{
    compute_escape_time(ITERATION_COUNT, 2.0, z, c, |z, c|
    {
        z.comp_squared() + c
    })
}

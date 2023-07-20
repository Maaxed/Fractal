use crate::complex::Complex;
use super::escape_time_method::compute_escape_time;

const ITERATION_COUNT: u32 = 1024;

pub fn mandelbrot_value(pos: Complex) -> f32
{
    // Cardioid / bulb checking
    let q = (pos + Complex::new(-0.25, 0.0)).modulus_squared();

    if q * (q + (pos.re() - 0.25)) <= 0.25 * pos.im() * pos.im() // the point is within the cardioid
        || (pos + Complex::new(1.0, 0.0)).modulus_squared() < 0.25 * 0.25 // the poitn is within the period-2 bulb
    {
        return 1.0
    }

    mandelbrot_base(Complex::ZERO, pos)
}

pub fn mandelbrot_julia_set(pos: Complex, secondary_pos: Complex) -> f32
{
    mandelbrot_base(pos, secondary_pos)
}

fn mandelbrot_base(z: Complex, c: Complex) -> f32
{
    compute_escape_time(ITERATION_COUNT, 2.0, z, c, |z, c|
    {
        z.squared() + c
    })
}

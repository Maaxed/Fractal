use crate::complex::Complex;

pub fn compute_escape_time(iteration_count: u32, max_length: f64, pos: Complex, secondary_pos: Complex, mut iteration_function: impl FnMut(Complex, Complex) -> Complex) -> f32
{
    let max_length_squared = max_length * max_length;
    let mut z = pos;
    let mut prev_z = z;
    for i in 0..iteration_count
    {
        let length_squared = z.modulus_squared();
        if length_squared > max_length_squared
        {
            return i as f32 / iteration_count as f32;
        }
        z = iteration_function(z, secondary_pos);

        // Periodicity checking: check for cycles with previously saved z
        if Complex::fuzzy_eq(z,  prev_z, 1.0e-20)
        {
            return 1.0;
        }

        // Save z every 32 iteration
        if i % 32 == 7
        {
            prev_z = z;
        }
    }

    1.0
}

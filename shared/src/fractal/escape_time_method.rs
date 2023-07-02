use glam::DVec2;

pub fn compute_escape_time(iteration_count: u32, max_length: f64, pos: DVec2, secondary_pos: DVec2, mut iteration_function: impl FnMut(DVec2, DVec2) -> DVec2) -> f32
{
    let max_length_squared = max_length * max_length;
    let mut z = pos;
    let mut prev_z = z;
    for i in 0..iteration_count
    {
        let length_squared = z.length_squared();
        if length_squared > max_length_squared
        {
            return i as f32 / iteration_count as f32;
        }
        z = iteration_function(z, secondary_pos);

        // Periodicity checking: check for cycles with previously saved z
        if (z - prev_z).abs().cmplt(DVec2::splat(1.0e-20)).all()
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

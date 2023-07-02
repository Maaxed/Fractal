use glam::DVec2;

pub fn compute_escape_time(iteration_count: u32, max_length: f64, pos: DVec2, secondary_pos: DVec2, mut iteration_function: impl FnMut(DVec2, DVec2) -> DVec2) -> f32
{
    let max_length_squared = max_length * max_length;
    let mut z = pos;
    for i in 0..iteration_count
    {
        let length_squared = z.length_squared();
        if length_squared > max_length_squared
        {
            return i as f32 / iteration_count as f32;
        }
        z = iteration_function(z, secondary_pos);
    }

    1.0
}

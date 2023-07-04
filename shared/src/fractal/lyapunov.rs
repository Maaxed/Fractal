use glam::DVec2;

#[cfg(target_arch = "spirv")]
use num_traits::Float;

const ITERATION_COUNT: usize = 1024;

pub fn lyapunov<const L: usize>(sequence: &[bool; L], pos: DVec2) -> f32
{
    if pos.x < 0.0 || -pos.y < 0.0 || pos.x > 4.0 || -pos.y > 4.0
    {
        return 0.0;
    }

    let mut x = 0.5;
    let mut lyapunov_exp = 0.0;

    for i in 0..ITERATION_COUNT
    {
        let rn = if sequence[i % sequence.len()] { -pos.y } else { pos.x };

        if i != 0
        {
            lyapunov_exp += ((rn * (1.0 - 2.0 * x)).abs() as f32).ln();
        }

        x = rn * x * (1.0 - x);
    }

    lyapunov_exp / ITERATION_COUNT as f32
}
use crate::math::*;

const ITERATION_COUNT: usize = 1024;

pub fn lyapunov<S: Scalar, const L: usize>(sequence: &[bool; L], pos: Vec2<S>) -> f32
{
    let zero = S::zero();
    let one = S::one();
    let two: S = 2.0_f32.into();
    let four = 4.0_f32.into();
    if pos.x() < zero || pos.y() < zero || pos.x() > four || pos.y() > four
    {
        return 0.0;
    }

    let mut x: S = 0.5_f32.into();
    let mut lyapunov_exp = 0.0;

    for i in 0..ITERATION_COUNT
    {
        let rn = if sequence[i % sequence.len()] { pos.y() } else { pos.x() };

        if i != 0
        {
            lyapunov_exp += ln(abs((rn * (one - two * x)).as_()));
        }

        x = rn * x * (one - x);
    }

    lyapunov_exp / ITERATION_COUNT as f32
}
// The area of a circle is defined as πr^2. Estimate π to 3 decimal places using a Monte Carlo method.
//
// Hint: The basic equation of a circle is x2 + y2 = r2.

use rand::{distributions::Uniform, prelude::Distribution};
const CIRCLE_RADIUS: u8 = 1;
const DECIMALS: f32 = 10000.0;

pub fn monte_carlo(n: u32) -> f32 {
    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(0..=DECIMALS as u32);
    let mut give_uniformly_distributed_float = || distribution.sample(&mut rng) as f32 / DECIMALS;

    ((0..n).fold(0, |points_in_circle: u32, _| {
        if give_uniformly_distributed_float().powi(2) + give_uniformly_distributed_float().powi(2)
            <= CIRCLE_RADIUS as f32
        {
            return points_in_circle + 1;
        }
        points_in_circle
    }) as f32
        / n as f32)
        * 4f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carlo() {
        println!("{}", monte_carlo(100000));
    }
}

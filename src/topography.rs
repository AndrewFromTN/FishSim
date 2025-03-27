use std::fmt::Display;
use std::vec::Vec;

use noise::{NoiseFn, Perlin};

pub struct TopographicMap {
    seed: u32,
    width: usize,
    height: usize,
    scale: f64,
    data: Vec<f64>
}

impl TopographicMap {
    pub fn new(seed: u32, width: usize, height: usize, scale: f64) -> Self {
        let data = generate(seed, width, height, scale);
        Self {
            seed,
            width,
            height,
            scale,
            data
        }
    }
}

fn generate(seed: u32, width: usize, height: usize, scale: f64) -> Vec<f64> {
    let perlin = Perlin::new(seed);
    let mut data = vec![0f64; width * height];

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let depth = perlin.get([nx, ny]);

            data[(y * width) + x] = depth;
        }
    }

    data
}

impl Display for TopographicMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let elem = self.data.get((y * self.width) + x).expect("Indexed element must exist");
                let symbol = match *elem {
                    d if d < -0.5 => '█',
                    d if d < -0.2 => '▓',
                    d if d < 0.0  => '▒',
                    d if d < 0.3  => '░',
                    _ => '#',
                };

                write!(f, "{}", symbol)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
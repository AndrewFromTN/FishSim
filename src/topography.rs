use colored::Colorize;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Display;
use std::vec::Vec;

use noise::{NoiseFn, Perlin};

const SUPER_SHALLOW: f64 = 0.75f64;
const SHALLOW: f64 = 0.5f64;
const MID: f64 = 0.0f64;
const DEEP: f64 = -0.6f64;

pub enum BottomComposition {
    Mud,
    Hard,
    Gravel,
}

pub enum Vegetation {
    Grass,
    Reeds,
    Mats,
}

impl Display for Vegetation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Vegetation::Grass => "„".green(),
            Vegetation::Reeds => "¥".green(),
            Vegetation::Mats => "¬".green(),
        };

        write!(f, "{}", text)
    }
}

pub enum Structure {
    ChunkRock,
    Boulder,
    Timber,
    Brush,
}

impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Structure::ChunkRock => "¤".red(),
            Structure::Boulder => "®".red(),
            Structure::Timber => "˜".yellow(),
            Structure::Brush => "×".yellow(),
        };

        write!(f, "{}", text)
    }
}

pub struct TopographicRegion {
    bottom: BottomComposition,
    vegetation: Option<Vegetation>,
    structure: Option<Structure>,
    depth: f64,
}

impl TopographicRegion {
    pub fn new(
        bottom: BottomComposition,
        vegetation: Option<Vegetation>,
        structure: Option<Structure>,
        depth: f64,
    ) -> Self {
        Self {
            bottom,
            vegetation,
            structure,
            depth,
        }
    }

    pub fn has_vegetation_type(&self, vegetation_type: &Vegetation) -> bool {
        if let Some(veg) = &self.vegetation {
            matches!(veg, vegetation_type)
        } else {
            false
        }
    }
}

impl Display for TopographicRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(veg) = &self.vegetation {
            write!(f, "{}", veg)
        } else if let Some(struc) = &self.structure {
            write!(f, "{}", struc)
        } else {
            let symbol = match self.depth {
                d if d < DEEP => "█",
                d if d < MID => "▓",
                d if d < SHALLOW => "▒",
                d if d < SUPER_SHALLOW => "░",
                _ => "#",
            };

            write!(f, "{}", symbol.blue())
        }
    }
}

pub struct TopographicMap {
    seed: u32,
    width: usize,
    height: usize,
    scale: f64,
    data: Vec<TopographicRegion>,
}

impl TopographicMap {
    pub fn new(seed: u32, width: usize, height: usize, scale: f64) -> Self {
        let data = generate(seed, width, height, scale);
        Self {
            seed,
            width,
            height,
            scale,
            data,
        }
    }
}

enum AdjacencyDirection {
    Up,
    Left,
}

fn get_adjacent(
    map: &[TopographicRegion],
    width: usize,
    x: usize,
    y: usize,
    direction: AdjacencyDirection,
) -> Option<&TopographicRegion> {
    if y == 0 && matches!(direction, AdjacencyDirection::Up) {
        return None;
    }

    if x == 0 && matches!(direction, AdjacencyDirection::Left) {
        return None;
    }

    let index = match direction {
        AdjacencyDirection::Up => ((y - 1) * width) + x,
        AdjacencyDirection::Left => (y * width) + x - 1,
    };

    Some(map.get(index).expect("Indexed element must exist"))
}

fn generate(seed: u32, width: usize, height: usize, scale: f64) -> Vec<TopographicRegion> {
    const REED_RATES: [f64; 4] = [0.2f64, 0.2f64, 0.0f64, 0.0f64];
    const ADJACENT_REED_RATES: [f64; 4] = [0.75f64, 0.4f64, 0.0f64, 0.0f64];

    const GRASS_RATES: [f64; 4] = [0.1f64, 0.2f64, 0.12f64, 0.05f64];
    const ADJACENT_GRASS_RATES: [f64; 4] = [0.45f64, 0.65f64, 0.45f64, 0.20f64];

    const MAT_RATES: [f64; 4] = [0.1f64, 0.2f64, 0.12f64, 0.05f64];
    const ADJACENT_MAT_RATES: [f64; 4] = [0.75f64, 0.75f64, 0.45f64, 0.20f64];

    const VEGETATIONS: [[f64; 4]; 3] = [REED_RATES, GRASS_RATES, MAT_RATES];
    const ADJACENT_VEGETATIONS: [[f64; 4]; 3] = [
        ADJACENT_REED_RATES,
        ADJACENT_GRASS_RATES,
        ADJACENT_MAT_RATES,
    ];

    const BASELINE_VEG_CHANCE: f64 = 0.75f64;

    let mut rng = ChaCha8Rng::seed_from_u64(seed.into());

    let perlin = Perlin::new(seed);
    let mut data = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let depth = perlin.get([nx, ny]);

            let mut vegetation: Option<Vegetation> = None;
            let mut structure: Option<Structure> = None;

            // Check that we are not on land
            if depth <= SUPER_SHALLOW {
                let up_adjacent = get_adjacent(&data, width, x, y, AdjacencyDirection::Up);
                let left_adjacent = get_adjacent(&data, width, x, y, AdjacencyDirection::Left);

                let veg_index = rng.random_range(..3);
                let chosen_veg_rates = VEGETATIONS[veg_index];
                let chosen_adjacent_veg_rates = ADJACENT_VEGETATIONS[veg_index];

                let veg_type = match veg_index {
                    0 => Vegetation::Grass,
                    1 => Vegetation::Reeds,
                    2 => Vegetation::Mats,
                    _ => unreachable!(),
                };

                let adjacent_vegetation = if let Some(up) = up_adjacent {
                    up.has_vegetation_type(&veg_type)
                } else if let Some(left) = left_adjacent {
                    left.has_vegetation_type(&veg_type)
                } else {
                    false
                };

                let depth_index = match depth {
                    d if d < DEEP => 3,
                    d if d < MID => 2,
                    d if d < SHALLOW => 1,
                    d if d < SUPER_SHALLOW => 0,
                    _ => unreachable!(),
                };

                let veg_random = rng.random_range(0..=100) as f64 / 100.0f64;
                let veg_depth_rate = if adjacent_vegetation {
                    chosen_adjacent_veg_rates[depth_index]
                } else {
                    chosen_veg_rates[depth_index]
                };

                // Determine if vegetation should exist based on depth
                if veg_random <= veg_depth_rate {
                    vegetation = Some(veg_type)
                }
            }

            let region =
                TopographicRegion::new(BottomComposition::Hard, vegetation, structure, depth);
            data.push(region);
        }
    }

    data
}

impl Display for TopographicMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let elem = self
                    .data
                    .get((y * self.width) + x)
                    .expect("Indexed element must exist");

                write!(f, "{}", *elem)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

use colored::Colorize;
use std::fmt::Display;
use std::vec::Vec;

use noise::{NoiseFn, Perlin};

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
}

impl Display for TopographicRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(veg) = &self.vegetation {
            write!(f, "{}", veg)
        } else if let Some(struc) = &self.structure {
            write!(f, "{}", struc)
        } else {
            let symbol = match self.depth {
                d if d < -0.5 => "█",
                d if d < -0.2 => "▓",
                d if d < 0.0 => "▒",
                d if d < 0.3 => "░",
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
) -> &TopographicRegion {
    let index = match direction {
        AdjacencyDirection::Up => ((y - 1) * width) + x,
        AdjacencyDirection::Left => (y * width) + x - 1,
    };

    map.get(index).expect("Indexed element must exist")
}

fn generate(seed: u32, width: usize, height: usize, scale: f64) -> Vec<TopographicRegion> {
    const REEDS_RATE: f64 = 0.25;
    const SHALLOW_GRASS_RATE: f64 = 0.5;
    const DEEP_GRASS_RATE: f64 = 0.2;
    const TIMBER_GRASS_RATE: f64 = 0.2;
    const ADJACENT_TIMBER_GRASS_RATE: f64 = TIMBER_GRASS_RATE / 2.0f64;
    const BOULDER_RATE: f64 = 0.05;
    const CHUNK_RATE: f64 = 0.25;
    const ADJACENT_CHUNK_RATE: f64 = CHUNK_RATE * 0.85;

    let perlin = Perlin::new(seed);
    let mut data = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let depth = perlin.get([nx, ny]);

            let region = TopographicRegion::new(BottomComposition::Hard, None, None, depth);
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

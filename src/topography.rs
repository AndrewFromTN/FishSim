use colored::Colorize;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Display;
use std::vec::Vec;

use noise::{NoiseFn, Perlin};

const DEPTH_MIN: f64 = 0.0f64;
const DEPTH_MAX: f64 = 15.0f64;
const NOISE_MIN: f64 = -1.0f64;
const NOISE_LAND_MIN: f64 = NOISE_MIN + 0.5f64; // (-0.5,-1.0] is considered land
const NOISE_MAX: f64 = 1.0f64;

pub struct DepthRange {
    pub min: f64,
    pub max: f64,
    pub vegetation_rates: [VegetationRate; 3],
    pub name: DepthRangeName,
}

impl DepthRange {
    pub fn get_vegetation_rate(&self, veg: &Vegetation, adjacent: bool) -> f64 {
        let rates = self
            .vegetation_rates
            .iter()
            .find(|x| matches!(&x.vegetation, veg))
            .expect("Vegetation must be present");

        if adjacent {
            rates.adjacency_rate
        } else {
            rates.rate
        }
    }
}

pub enum DepthRangeName {
    SuperShallow,
    Shallow,
    MidDepth,
    Deep,
}

impl Display for DepthRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self.name {
            DepthRangeName::Deep => "█",
            DepthRangeName::MidDepth => "▓",
            DepthRangeName::Shallow => "▒",
            DepthRangeName::SuperShallow => "░",
        };

        write!(f, "{}", symbol.blue())
    }
}

pub const DEPTH_RANGES: [DepthRange; 4] = [
    DepthRange {
        min: DEPTH_MIN,
        max: 5.0f64,
        vegetation_rates: [
            VegetationRate {
                vegetation: Vegetation::Grass,
                rate: 0.1f64,
                adjacency_rate: 0.45f64,
            },
            VegetationRate {
                vegetation: Vegetation::Reeds,
                rate: 0.2f64,
                adjacency_rate: 0.75f64,
            },
            VegetationRate {
                vegetation: Vegetation::Mats,
                rate: 0.1f64,
                adjacency_rate: 0.75f64,
            },
        ],
        name: DepthRangeName::SuperShallow,
    },
    DepthRange {
        min: 5.0f64,
        max: 7.0f64,
        vegetation_rates: [
            VegetationRate {
                vegetation: Vegetation::Grass,
                rate: 0.2f64,
                adjacency_rate: 0.65f64,
            },
            VegetationRate {
                vegetation: Vegetation::Reeds,
                rate: 0.2f64,
                adjacency_rate: 0.4f64,
            },
            VegetationRate {
                vegetation: Vegetation::Mats,
                rate: 0.2f64,
                adjacency_rate: 0.75f64,
            },
        ],
        name: DepthRangeName::Shallow,
    },
    DepthRange {
        min: 7.0f64,
        max: 10.0f64,
        vegetation_rates: [
            VegetationRate {
                vegetation: Vegetation::Grass,
                rate: 0.12f64,
                adjacency_rate: 0.45f64,
            },
            VegetationRate {
                vegetation: Vegetation::Reeds,
                rate: 0.0f64,
                adjacency_rate: 0.0f64,
            },
            VegetationRate {
                vegetation: Vegetation::Mats,
                rate: 0.12f64,
                adjacency_rate: 0.45f64,
            },
        ],
        name: DepthRangeName::MidDepth,
    },
    DepthRange {
        min: 10.0f64,
        max: DEPTH_MAX,
        vegetation_rates: [
            VegetationRate {
                vegetation: Vegetation::Grass,
                rate: 0.05f64,
                adjacency_rate: 0.20f64,
            },
            VegetationRate {
                vegetation: Vegetation::Reeds,
                rate: 0.0f64,
                adjacency_rate: 0.0f64,
            },
            VegetationRate {
                vegetation: Vegetation::Mats,
                rate: 0.05f64,
                adjacency_rate: 0.20f64,
            },
        ],
        name: DepthRangeName::Deep,
    },
];

#[derive(Debug, Clone)]
pub struct DepthError;

impl Display for DepthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid depth")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoiseDepth(f64);

impl NoiseDepth {
    pub fn is_land(&self) -> bool {
        self.0 < NOISE_LAND_MIN
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Depth(f64);

impl Depth {
    fn depth_range(&self) -> &DepthRange {
        DEPTH_RANGES
            .iter()
            .find(|x| self.0 >= x.min && self.0 <= x.max)
            .expect("Depth range must exist")
    }
}

impl From<NoiseDepth> for Depth {
    fn from(noise_value: NoiseDepth) -> Self {
        let converted_value = match noise_value.is_land() {
            true => DEPTH_MIN - 1.0f64,
            false => {
                (noise_value.0 - NOISE_LAND_MIN) / (NOISE_MAX - NOISE_LAND_MIN)
                    * (DEPTH_MAX - DEPTH_MIN)
                    + DEPTH_MIN
            }
        };

        Self(converted_value)
    }
}

impl Display for Depth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.depth_range())
    }
}

pub enum BottomComposition {
    Mud,
    Hard,
    Gravel,
}

pub struct VegetationRate {
    vegetation: Vegetation,
    rate: f64,
    adjacency_rate: f64,
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

pub enum TopographicRegion {
    Land(TopographicLandRegion),
    Water(TopographicWaterRegion),
}

impl Display for TopographicRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Land(land) => land.fmt(f),
            Self::Water(water) => water.fmt(f),
        }
    }
}

pub struct TopographicLandRegion {}

impl Display for TopographicLandRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")
    }
}

pub struct TopographicWaterRegion {
    bottom: BottomComposition,
    vegetation: Option<Vegetation>,
    structure: Option<Structure>,
    depth: Depth,
}

impl TopographicWaterRegion {
    pub fn new(
        bottom: BottomComposition,
        vegetation: Option<Vegetation>,
        structure: Option<Structure>,
        depth: Depth,
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

impl Display for TopographicWaterRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(veg) = &self.vegetation {
            write!(f, "{}", veg)
        } else if let Some(struc) = &self.structure {
            write!(f, "{}", struc)
        } else {
            write!(f, "{}", self.depth)
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
    let mut rng = ChaCha8Rng::seed_from_u64(seed.into());

    let perlin = Perlin::new(seed);
    let mut data = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let noise_depth = NoiseDepth(perlin.get([nx, ny]));

            if noise_depth.is_land() {
                data.push(TopographicRegion::Land(TopographicLandRegion {}));
            } else {
                let depth = Depth::from(noise_depth);

                let mut vegetation: Option<Vegetation> = None;
                let mut structure: Option<Structure> = None;

                let up_adjacent = get_adjacent(&data, width, x, y, AdjacencyDirection::Up);
                let left_adjacent = get_adjacent(&data, width, x, y, AdjacencyDirection::Left);

                let veg_type = match rng.random_range(0..3) {
                    0 => Vegetation::Grass,
                    1 => Vegetation::Reeds,
                    2 => Vegetation::Mats,
                    _ => unreachable!(),
                };

                let adjacent_vegetation = if let Some(up) = up_adjacent {
                    match up {
                        TopographicRegion::Land(_) => false,
                        TopographicRegion::Water(water) => water.has_vegetation_type(&veg_type),
                    }
                } else if let Some(left) = left_adjacent {
                    match left {
                        TopographicRegion::Land(_) => false,
                        TopographicRegion::Water(water) => water.has_vegetation_type(&veg_type),
                    }
                } else {
                    false
                };

                let vegetation_rate = depth
                    .depth_range()
                    .get_vegetation_rate(&veg_type, adjacent_vegetation);

                let veg_random = rng.random_range(0..=100) as f64 / 100.0f64;
                if veg_random <= vegetation_rate {
                    vegetation = Some(veg_type)
                }

                let region = TopographicRegion::Water(TopographicWaterRegion::new(
                    BottomComposition::Hard,
                    vegetation,
                    structure,
                    depth,
                ));
                data.push(region);
            }
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

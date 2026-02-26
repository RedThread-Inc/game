use noise::{NoiseFn, Perlin};

pub const DEBUG_SEED: u32 = 42;

const WATER_COVERAGE: f64 = 0.12;
const DIRT_BAND: f64 = 0.07;
const WATER_DIRT_RADIUS: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainZone {
    Water,
    Dirt,
    GreenGrass,
    YellowGrass,
}

pub struct HeightMap {
    values: Vec<f64>,
    zones: Vec<TerrainZone>,
    pub width: u32,
    pub height: u32,
    water_threshold: f64,
    dirt_threshold: f64,
}

impl HeightMap {
    pub fn generate(width: u32, height: u32, seed: u32, _scale: f64) -> Self {
        let size = (width * height) as usize;
        let mut raw_values = Vec::with_capacity(size);

        let p0 = Perlin::new(seed);
        let p1 = Perlin::new(seed.wrapping_add(1));
        let p2 = Perlin::new(seed.wrapping_add(2));
        let p3 = Perlin::new(seed.wrapping_add(3));

        let base_scale = 2.2;

        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 / width as f64;
                let ny = y as f64 / height as f64;

                let v = p0.get([nx * base_scale,         ny * base_scale        ]) * 0.60
                    + p1.get([nx * base_scale * 2.5,   ny * base_scale * 2.5  ]) * 0.25
                    + p2.get([nx * base_scale * 5.0,   ny * base_scale * 5.0  ]) * 0.12
                    + p3.get([nx * base_scale * 10.0,  ny * base_scale * 10.0 ]) * 0.03;

                raw_values.push(((v + 1.0) / 2.0).clamp(0.0, 1.0));
            }
        }

        let smoothed = smooth(&raw_values, width, height);
        let (water_threshold, dirt_threshold) =
            compute_thresholds(&smoothed, WATER_COVERAGE, WATER_COVERAGE + DIRT_BAND);

        println!(
            "[Perlin] water_threshold={:.3}, dirt_threshold={:.3}",
            water_threshold, dirt_threshold
        );

        let mut zones: Vec<TerrainZone> = smoothed
            .iter()
            .map(|&v| classify_value(v, water_threshold, dirt_threshold))
            .collect();

        apply_water_border(&mut zones, width, height, WATER_DIRT_RADIUS);

        Self {
            values: smoothed,
            zones,
            width,
            height,
            water_threshold,
            dirt_threshold,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> f64 {
        self.values[(y * self.width + x) as usize]
    }

    pub fn classify(&self, x: u32, y: u32) -> TerrainZone {
        self.zones[(y * self.width + x) as usize]
    }
}

fn classify_value(v: f64, water_threshold: f64, dirt_threshold: f64) -> TerrainZone {
    if v < water_threshold {
        TerrainZone::Water
    } else if v < dirt_threshold {
        TerrainZone::Dirt
    } else {
        TerrainZone::GreenGrass
    }
}

fn apply_water_border(zones: &mut Vec<TerrainZone>, width: u32, height: u32, radius: i32) {
    let w = width as i32;
    let h = height as i32;

    let water_positions: Vec<(i32, i32)> = zones
        .iter()
        .enumerate()
        .filter(|(_, z)| **z == TerrainZone::Water)
        .map(|(i, _)| ((i as i32) % w, (i as i32) / w))
        .collect();

    for (wx, wy) in water_positions {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let nx = wx + dx;
                let ny = wy + dy;
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }
                let idx = (ny * w + nx) as usize;
                if zones[idx] != TerrainZone::Water {
                    zones[idx] = TerrainZone::Dirt;
                }
            }
        }
    }
}

fn compute_thresholds(values: &[f64], p_water: f64, p_dirt: f64) -> (f64, f64) {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();

    let water_idx = ((p_water * n as f64) as usize).clamp(0, n - 1);
    let dirt_idx  = ((p_dirt  * n as f64) as usize).clamp(0, n - 1);

    (sorted[water_idx], sorted[dirt_idx])
}

fn smooth(values: &[f64], width: u32, height: u32) -> Vec<f64> {
    let mut out = vec![0.0f64; values.len()];
    let w = width as i32;
    let h = height as i32;
    let kernel: [(i32, i32, f64); 9] = [
        (-1, -1, 1.0), (0, -1, 2.0), (1, -1, 1.0),
        (-1,  0, 2.0), (0,  0, 4.0), (1,  0, 2.0),
        (-1,  1, 1.0), (0,  1, 2.0), (1,  1, 1.0),
    ];

    for y in 0..h {
        for x in 0..w {
            let mut sum = 0.0;
            let mut weight_sum = 0.0;
            for (dx, dy, weight) in &kernel {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < w && ny >= 0 && ny < h {
                    sum += values[(ny * w + nx) as usize] * weight;
                    weight_sum += weight;
                }
            }
            out[(y * w + x) as usize] = sum / weight_sum;
        }
    }

    out
}
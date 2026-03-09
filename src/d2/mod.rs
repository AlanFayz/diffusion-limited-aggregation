use rand::{RngExt, rngs::ThreadRng};
use std::{
    fs::{File, exists},
    io::Write,
};

use crate::utils::{lerp, pack_rgba, unpack_rgba};

#[derive(Default, Clone, Copy)]
struct Cell {
    occupied: bool,
    iter_count: Option<usize>,
}

fn random_position(rng: &mut ThreadRng, width: usize, height: usize) -> (usize, usize) {
    (
        rng.random::<u32>() as usize % width,
        rng.random::<u32>() as usize % height,
    )
}

fn count_neighbours(x: i64, y: i64, width: i64, height: i64, grid: &Vec<Cell>) -> u32 {
    let mut cnt = 0;

    for dy in -1..2 {
        for dx in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let idx = (x + dx).rem_euclid(width) + (y + dy).rem_euclid(height) * width;
            if grid[idx as usize].occupied {
                cnt += 1;
            }
        }
    }

    return cnt;
}

fn shade(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    particle_count: usize,
    grid: &Vec<Cell>,
) -> u32 {
    const BG_COLOR: u32 = 0x0;
    const START_COLOR: u32 = 0xFF0000FF;
    const END_COLOR: u32 = 0x0000FFFF;
    const MAX_NEIGHBOURS: f32 = 8.0;

    let cell = &grid[x + y * width];

    if !cell.occupied {
        return BG_COLOR;
    }

    let (s_r, s_g, s_b) = unpack_rgba(START_COLOR);
    let (e_r, e_g, e_b) = unpack_rgba(END_COLOR);

    let t = cell.iter_count.unwrap_or(0) as f32 / particle_count as f32;

    let r = lerp(s_r, e_r, t);
    let g = lerp(s_g, e_g, t);
    let b = lerp(s_b, e_b, t);

    let cnt = count_neighbours(x as i64, y as i64, width as i64, height as i64, grid);
    let intensity = 1.0 - (cnt as f32 / MAX_NEIGHBOURS).powf(1.5).clamp(0.0, 1.0);

    return pack_rgba(r * intensity, g * intensity, b * intensity);
}

pub fn run_simulation(
    out_path: &str,
    width: usize,
    height: usize,
    particle_count: usize,
) -> Option<()> {
    if !exists(out_path).ok()? {
        return None;
    }

    let mut rng = rand::rng();

    let seed = random_position(&mut rng, width, height);

    let mut grid: Vec<Cell> = vec![Default::default(); width * height];

    grid[seed.0 + seed.1 * width].occupied = true;
    grid[seed.0 + seed.1 * width].iter_count = Some(0);

    for i in 0..particle_count {
        let (mut x, mut y) = random_position(&mut rng, width, height);
        println!(
            "Progress: {}%",
            (i as f32) * 100.0 / (particle_count as f32)
        );

        loop {
            if count_neighbours(x as i64, y as i64, width as i64, height as i64, &grid) > 0 {
                break;
            }

            let nxt = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let (dx, dy) = nxt[rng.random::<i64>() as usize % nxt.len()];
            let (nx, ny) = (x as i64 + dx, y as i64 + dy);
            (x, y) = (
                nx.rem_euclid(width as i64) as usize,
                ny.rem_euclid(height as i64) as usize,
            );
        }

        grid[x + y * width].occupied = true;
        grid[x + y * width].iter_count = Some(i + 1);
    }

    let mut file = File::create(out_path).unwrap();
    file.write(format!("P6\n{} {}\n255\n", width, height).as_bytes())
        .ok()?;

    for y in 0..height {
        for x in 0..width {
            file.write(&shade(x, y, width, height, particle_count, &grid).to_be_bytes()[0..3])
                .ok()?;
        }
    }

    return Some(());
}

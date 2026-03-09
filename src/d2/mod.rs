use rand::{RngExt, rngs::ThreadRng};
use std::{
    fs::{File, exists},
    io::Write,
    time::Instant,
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

fn count_neighbours_radius(
    x: i64,
    y: i64,
    width: i64,
    height: i64,
    radius: i64,
    grid: &Vec<Cell>,
) -> u32 {
    let mut cnt = 0;

    for dy in -radius..radius + 1 {
        for dx in -radius..radius + 1 {
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

fn count_neighbours(x: i64, y: i64, width: i64, height: i64, grid: &Vec<Cell>) -> u32 {
    count_neighbours_radius(x, y, width, height, 1, grid)
}

fn shade(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    particle_count: usize,
    grid: &Vec<Cell>,
    bg_color: u32,
    start_color: u32,
    end_color: u32,
) -> u32 {
    const NEIGHBOURS_RADIUS: u32 = 5;
    const MAX_NEIGHBOURS: f32 =
        ((NEIGHBOURS_RADIUS * 2 + 1) * (NEIGHBOURS_RADIUS * 2 + 1)) as f32 - 1.0;

    let cell = &grid[x + y * width];

    if !cell.occupied {
        return bg_color;
    }

    let (s_r, s_g, s_b) = unpack_rgba(start_color);
    let (e_r, e_g, e_b) = unpack_rgba(end_color);

    let t = cell.iter_count.unwrap_or(0) as f32 / particle_count as f32;

    let r = lerp(s_r, e_r, t);
    let g = lerp(s_g, e_g, t);
    let b = lerp(s_b, e_b, t);

    let cnt = count_neighbours_radius(
        x as i64,
        y as i64,
        width as i64,
        height as i64,
        NEIGHBOURS_RADIUS as i64,
        grid,
    );
    let intensity = 1.0 - (cnt as f32 / MAX_NEIGHBOURS);

    return pack_rgba(r * intensity, g * intensity, b * intensity);
}

pub fn run_simulation(
    out_path: &str,
    width: usize,
    height: usize,
    particle_count: usize,
    progress_check: bool,
    profile: bool,
    check_scale: usize,
    bg_color: u32,
    start_color: u32,
    end_color: u32,
) -> Option<()> {
    let check_width = width.div_ceil(check_scale);
    let check_height = height.div_ceil(check_scale);

    let mut rng = rand::rng();
    let mut grid: Vec<Cell> = vec![Default::default(); width * height];
    let mut check_grid: Vec<bool> = vec![false; check_width * check_height];

    let seed = random_position(&mut rng, width, height);

    grid[seed.0 + seed.1 * width].occupied = true;
    grid[seed.0 + seed.1 * width].iter_count = Some(0);

    check_grid[(seed.0 / check_scale) + (seed.1 / check_scale) * check_width] = true;

    let start = Instant::now();

    for i in 0..particle_count {
        let (mut x, mut y) = random_position(&mut rng, width, height);

        if progress_check {
            println!(
                "Progress: {}%",
                (i as f32) * 100.0 / (particle_count as f32)
            );
        }

        loop {
            if count_neighbours(x as i64, y as i64, width as i64, height as i64, &grid) > 0 {
                break;
            }

            let step_size = 'sz: {
                let (cx, cy) = (x / check_scale, y / check_scale);
                for dy in -1..2 {
                    for dx in -1..2 {
                        let (nx, ny) = (cx as i64 + dx, cy as i64 + dy);
                        let (cx, cy) = (
                            nx.rem_euclid(check_width as i64) as usize,
                            ny.rem_euclid(check_height as i64) as usize,
                        );

                        if check_grid[cx + cy * check_width] == true {
                            break 'sz 1;
                        }
                    }
                }

                check_scale as i64
            };

            let nxt = [
                (0, -1 * step_size),
                (0, 1 * step_size),
                (-1 * step_size, 0),
                (1 * step_size, 0),
            ];
            let (dx, dy) = nxt[rng.random::<i64>() as usize % nxt.len()];
            let (nx, ny) = (x as i64 + dx, y as i64 + dy);
            (x, y) = (
                nx.rem_euclid(width as i64) as usize,
                ny.rem_euclid(height as i64) as usize,
            );
        }

        grid[x + y * width].occupied = true;
        grid[x + y * width].iter_count = Some(i + 1);

        check_grid[(x / check_scale) + (y / check_scale) * check_width] = true;
    }

    let delta = Instant::now() - start;
    if profile {
        println!("Simulation took {}s", delta.as_secs_f64());
    }

    let mut file = File::create(out_path).unwrap();
    file.write(format!("P6\n{} {}\n255\n", width, height).as_bytes())
        .ok()?;

    for y in 0..height {
        for x in 0..width {
            file.write(
                &shade(
                    x,
                    y,
                    width,
                    height,
                    particle_count,
                    &grid,
                    bg_color,
                    start_color,
                    end_color,
                )
                .to_be_bytes()[0..3],
            )
            .ok()?;
        }
    }

    return Some(());
}

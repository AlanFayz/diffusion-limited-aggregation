use rand::{RngExt, rngs::ThreadRng};
use std::{fs::File, io::Write};

fn random_position(rng: &mut ThreadRng, width: usize, height: usize) -> (usize, usize) {
    (
        rng.random::<u32>() as usize % width,
        rng.random::<u32>() as usize % height,
    )
}

fn check_bounds(
    x: i64,
    y: i64,
    width: i64,
    height: i64,
    particle_color: u32,
    grid: &Vec<u32>,
) -> bool {
    for dy in -1..2 {
        if y + dy < 0 || y + dy >= height {
            continue;
        }
        for dx in -1..2 {
            if x + dx < 0 || x + dx >= width {
                continue;
            }
            let idx = (x + dx) + (y + dy) * width;
            if grid[idx as usize] == particle_color {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let mut rng = rand::rng();

    let (width, height) = (400usize, 400usize);
    let particle_count = 100 * 100;
    let seed = random_position(&mut rng, width, height);

    let bg_color: u32 = 0xFFFFFFFF;
    let particle_color: u32 = 0xFF00FFFF;

    let mut grid: Vec<u32> = vec![bg_color; width * height];
    grid[seed.0 + seed.1 * width] = particle_color;

    for _ in 0..particle_count {
        let (mut x, mut y) = random_position(&mut rng, width, height);
        loop {
            if check_bounds(
                x as i64,
                y as i64,
                width as i64,
                height as i64,
                particle_color,
                &grid,
            ) {
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

        grid[x + y * width] = particle_color;
    }

    let mut file = File::create("out.ppm").unwrap();
    file.write(format!("P6\n{} {}\n255\n", width, height).as_bytes())
        .unwrap();

    for y in 0..height {
        for x in 0..width {
            let index = x + y * width;
            file.write(&grid[index].to_be_bytes()[0..3]).unwrap();
        }
    }
}

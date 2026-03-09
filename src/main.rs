use clap::Parser;
use rand::{
    Rng, RngExt, SeedableRng,
    rngs::{StdRng, SysRng},
};

mod d2;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "out.ppm")]
    file_path: String,

    #[arg(long, default_value_t = 1000)]
    width: usize,

    #[arg(long, default_value_t = 1000)]
    height: usize,

    #[arg(long, default_value_t = 200*200)]
    particle_count: usize,

    #[arg(long, default_value_t = true)]
    progress_check: bool,

    #[arg(long, default_value_t = true)]
    profile: bool,

    #[arg(long, default_value_t = 32)]
    check_scale: usize,

    #[arg(long, value_parser = parse_hex, default_value = "random")]
    start_color: u32,

    #[arg(long, value_parser = parse_hex, default_value = "random")]
    end_color: u32,

    #[arg(long, value_parser = parse_hex, default_value = "0x0")]
    bg_color: u32,
}

fn parse_hex(s: &str) -> Result<u32, String> {
    let mut rng = StdRng::try_from_rng(&mut SysRng).map_err(|_| format!("failed to create rng"))?;
    if s == "random" {
        return Ok(rng.random::<u32>());
    }

    let trimmed = s.trim_start_matches("0x").trim_start_matches('#');

    u32::from_str_radix(trimmed, 16).map_err(|_| {
        format!(
            "`{}` is not a valid 32-bit hexadecimal number, another option is also \"random\"",
            s
        )
    })
}

fn main() {
    let args = Args::parse();

    d2::run_simulation(
        &args.file_path,
        args.width,
        args.height,
        args.particle_count,
        args.progress_check,
        args.profile,
        args.check_scale,
        args.bg_color,
        args.start_color,
        args.end_color,
    )
    .unwrap();

    println!("Output written to {}", args.file_path);
}

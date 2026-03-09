use clap::Parser;

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
}

fn main() {
    let args = Args::parse();

    d2::run_simulation(
        &args.file_path,
        args.width,
        args.height,
        args.particle_count,
        args.progress_check,
        args.profile
    )
    .unwrap();

    println!("Output written to {}", args.file_path);
}

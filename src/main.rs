use clap::Parser;

mod d2;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "out.ppm")]
    file_path: String,

    #[arg(short, long, default_value_t = 200)]
    width: usize,

    #[arg(short, long, default_value_t = 200)]
    height: usize,

    #[arg(short, long, default_value_t = 50*100)]
    particle_count: usize,
}

fn main() {
    let args = Args::parse();

    d2::run_simulation(
        &args.file_path,
        args.width,
        args.height,
        args.particle_count,
    )
    .unwrap();

    println!("Output written to {}", args.file_path);
}

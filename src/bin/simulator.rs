use std::path::PathBuf;
use clap::Parser;

use cereal::simulator::{run, Options};

#[derive(Parser)]
struct Args {
    input_paths: Vec<PathBuf>,
    #[clap(long)]
    trace_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let options = Options {
        input_paths: args.input_paths,
        trace_path: args.trace_path,
        step_cap: None,
    };
    run(options);
}

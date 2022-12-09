// @Todo error handling, use span information
// @Todo indicate error in process return value
// @Todo debug info: .loc and filename indices
// @Todo name clashes and tests that fail
// @Todo change Blocks in backend to not be an enum, but to seperate vecs

use std::path::PathBuf;

#[derive(clap::Parser)]
struct Args {
    #[clap(default_value = "output.obj", long, short)]
    output_path: PathBuf,
    #[clap(long, short = 'g')]
    debug_info: bool,
    input_paths: Vec<PathBuf>,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    if args.input_paths.is_empty() {
        return;
    }

    let options = cereal::Options {
        output_path: args.output_path,
        debug_info: args.debug_info,
        input_paths: args.input_paths,
    };

    cereal::compile(options).expect("No compile fail");
}

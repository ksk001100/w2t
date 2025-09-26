use seahorse::{App, Context, Flag, FlagType};
use std::env;
use w2t_core::process_image_from_path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("w2t <input_path> <output_path> [--threshold <value>]")
        .flag(
            Flag::new("threshold", FlagType::Uint)
                .description("Set the threshold for transparency (default: 240)")
                .alias("t")
                .alias("th"),
        )
        .action(|c| run(c));

    app.run(args);
}

fn run(c: &Context) {
    if c.args.len() < 2 {
        eprintln!("Usage: w2t <input_path> <output_path>");
        std::process::exit(1);
    }
    let input = &c.args[0];
    let output = &c.args[1];
    let threshold: u8 = match &c.uint_flag("threshold") {
        Ok(t) => {
            if 255 >= *t {
                (*t).try_into().unwrap()
            } else {
                eprintln!("Threshold must be between 0 and 255");
                std::process::exit(1);
            }
        }
        Err(_) => 240,
    };
    if let Err(e) = process_image_from_path(input, output, threshold) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

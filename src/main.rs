use image::RgbaImage;
use seahorse::{App, Context, Flag, FlagType};
use std::path::Path;

fn main() {
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
        .action(|c: &Context| {
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
            if let Err(e) = process_image(input, output, threshold) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        });

    app.run(std::env::args().collect());
}

fn process_image<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    threshold: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?.to_rgba8();
    let (w, h) = img.dimensions();
    let mut out: RgbaImage = RgbaImage::new(w, h);

    for (x, y, px) in img.enumerate_pixels() {
        let [r, g, b, a] = px.0;
        let new_a = if r >= threshold && g >= threshold && b >= threshold {
            0
        } else {
            a
        };
        out.put_pixel(x, y, image::Rgba([r, g, b, new_a]));
    }
    out.save(output_path)?;
    Ok(())
}

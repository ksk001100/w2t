# w2t

w2t is a command-line tool that converts white (or bright) areas in an image to transparent, saving the result as a PNG with transparency.

## Features

- Automatically makes white areas of the input image transparent
- Adjustable threshold for transparency (default: 240)
- Simple CLI interface

## Installation

Make sure you have Rust installed, then build with:

```sh
cargo build --release
```

## Usage

```sh
w2t <input_path> <output_path> [--threshold <value>]
```

- `<input_path>`: Path to the input image file
- `<output_path>`: Path to the output PNG file
- `--threshold`, `-t`, `-th`: Threshold for transparency (0–255, default: 240)

### Example

```sh
w2t input.jpg output.png --threshold 250
```
In this example, pixels with RGB values greater than or equal to 250 will become transparent.

```sh
for t in {0..255}; do w2t input.jpg threshold_${t}.png -t $t; done
```
This command will create 256 PNG files with different transparency thresholds, ranging from 0 to 255.


## License

MIT

## Author

ksk001100 (<hm.pudding0715@gmail.com>)
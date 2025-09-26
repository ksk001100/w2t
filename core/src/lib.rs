use image::{DynamicImage, ImageResult, Rgba, RgbaImage};

pub fn make_transparent(img: &DynamicImage, threshold: u8) -> RgbaImage {
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    let mut transparent_img = RgbaImage::new(width, height);

    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let Rgba([r, g, b, a]) = *pixel;

        // 明るさを計算（簡単なグレースケール変換）
        let brightness = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) as u8;

        if brightness >= threshold {
            // 閾値以上の明るさの場合は透明にする
            transparent_img.put_pixel(x, y, Rgba([r, g, b, 0]));
        } else {
            // そうでなければ元の色を保持
            transparent_img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    transparent_img
}

pub fn process_image_from_path(
    input_path: &str,
    output_path: &str,
    threshold: u8,
) -> ImageResult<()> {
    let img = image::open(input_path)?;
    let transparent_img = make_transparent(&img, threshold);
    transparent_img.save(output_path)?;
    Ok(())
}

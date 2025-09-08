use crate::concurrency::mandelbrot::escape_time;
use crate::concurrency::parse::pixel_to_point;
use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;
use std::fs::File;

/// 把一个矩形区域内的曼德勃罗集渲染到像素的缓冲区里。
///
/// `bounds`参数指定了缓冲区`pixels`的宽度和高度，每个字节存储一个
/// 灰度像素。`upper_left`和`lower_right`参数指定了复平面中对应
/// 像素缓冲区左上角和右下角的两个点。
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// 把缓冲区`pixels`写入到文件`filename`，它的宽和高由`bounds`指定。
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}

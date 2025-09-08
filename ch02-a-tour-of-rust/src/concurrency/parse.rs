use num::Complex;
use std::str::FromStr;

/// 将`s`解析为一个坐标对，例如`"400x6000"`或者`"1.0,0.5"`
///
/// 确切地说，`s`的形式应该是 <left><sep><right>，其中 <sep> 是由
/// `separator`参数指定的字符，<left> 和 <right> 都是可以被`T::from_str`
/// 解析的字符串，`separator`必须是个 ASCII 字符。
///
/// 如果`s`的格式正确，就返回`Some<(x, y)>`。
/// 如果不能正确解析，就返回`None`。
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        // 可以直接match俩
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// 把一对逗号分隔的浮点数解析为一个复数
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// 给定输出图片中一个像素的行和列，返回复平面中相应的点。
///
/// `bounds`是一个指定图片的宽和高的值对。
/// `pixel`是一个（行，列）对，指定图片中的某个像素。
/// `upper_left`和`lower_right`参数是复平面上的点，
/// 指定我们的图像覆盖的区域。
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // 为什么这里是减法？当我们向下时 pixel.1会增大，但虚部会减小。
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse_complex("1.25,-0.0625"),
            Some(Complex {
                re: 1.25,
                im: -0.0625
            })
        );
        assert_eq!(
            parse_complex("1,-0.0625"),
            Some(Complex {
                re: 1.0,
                im: -0.0625
            })
        );
        assert_eq!(parse_complex(",-0.0625"), None);
    }

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 200),
                (25, 175),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}

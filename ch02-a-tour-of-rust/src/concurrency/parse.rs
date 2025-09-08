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
}

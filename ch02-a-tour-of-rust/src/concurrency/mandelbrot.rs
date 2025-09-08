use num::Complex;

/// 尝试判断`c`是否在曼德勃罗集里，最多迭代`limit`次。
///
/// 如果`c`不在曼德勃罗集里，就返回`Some(i)`，其中`i`是
/// `z`离开以原点为圆心、2 为半径的圆所需的迭代次数。
/// 如果`c`似乎在曼德勃罗集里（更确切地说是迭代了`limit`次
/// 之后仍无法证明`c`不在曼德勃罗集里），就返回`None`。
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

pub fn complex_square_add_loop(c: Complex<f64>) -> Complex<f64> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for _ in 0..10 {
        z = z * z + c;
    }
    z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_square_add_loop() {
        println!("{}", complex_square_add_loop(Complex { re: 1.0, im: 1.0 },));
    }

    #[test]
    fn test_escape_time() {
        println!("{:?}", escape_time(Complex { re: 1.0, im: 1.0 }, 10))
    }
}

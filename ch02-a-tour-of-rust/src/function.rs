/// 使用欧几里得算法计算两个整数的最大公约数
#[allow(dead_code)]
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// 编写并运行单元测试
// 函 数上方的 #[test] 标记 test_gcd 是一个测试函数，这种函数在正常编译时会被跳过
// 但在使用 cargo test 命令时会被编译并自动调用。
#[test]
fn test_gcd() {
    let n = 9;
    let m = 6;

    println!("gcd({},{}) = {}", n, m, gcd(n, m));

    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

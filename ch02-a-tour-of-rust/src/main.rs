use ch02::concurrency::draw::{render, write_image};
use ch02::concurrency::parse::{parse_complex, parse_pair};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/*fn main() {
    use ch02::function::gcd;
    use std::str::FromStr;

    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // 当我们在迭代时，我们想告诉Rust这个vector的 所有权 仍然属于numbers，我们只是 借用 它的值来进行循环。
    // &numbers[1..]中的&运算符借用了vector中从第二个元素开始到最后一个元素的引用。
    // for循环迭代引用的那些元素，每次迭代中用m借用每一个元素。*m中的*运算符 解引用 了m，返回了它所指向的值，
    // 也就是我们传递给gcd的第二个值。最后，因为numbers拥有vector的所有权，
    // 当numbers离开main的作用域时Rust会自动释放它的内存。
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}*/

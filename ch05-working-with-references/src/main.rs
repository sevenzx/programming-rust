fn main() {
    // 1. 解引用
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!(anime_ref.bechdel_pass, true);

    // 等价于上面的代码，但显式写出了解引用
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    // 2. 引用的引用
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.x, 1000);
    assert_eq!(rrr.y, 729);

    // 3. 比较引用
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    assert_eq!(rrx, rry);
    // 这里，最后的断言会成功，尽管rrx 和 rry 指向不同的值（rx 和 ry），但因为== 运算符会解
    // 除所有的引用然后对最终的值 x 和 y 进行比较。这几乎总是你想要的行为，尤其是编写泛型函数时。
    // 如果你实际上是想知道两个引用是否指向相同的内存位置，
    // 你可以使用std::ptr::eq，它会按照地址比较引用：
    assert!(!std::ptr::eq(rx, ry));

    // 注意比较运算符的操作数的类型必须完全相同，包括引用：
    // assert!(rx == rrx); // error：类型不匹配：`&i32` 和 `&&i32`
    assert!(rx == *rrx); // Ok
}

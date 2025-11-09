use std::rc::Rc;

fn main() {
    reference_count();
    copy_demo();
    move_demo();
}

/// reference_count 操作
fn reference_count() {
    let s: Rc<String> = Rc::new("seven".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = t.clone();
    println!("s = {}, t = {}, u = {}", s, t, u);

    assert!(s.contains("even"));
    assert_eq!(s.find("ve"), Some(2));

    // 一个 Rc 指针拥有的值是不可变的。假设你尝试在字符串的结尾添加文本:
    // u.push_str("eleven"); Rust将会报错
}

/// copy 操作
fn copy_demo() {
    // 标准的 Copy 类型包括所有的机器整数和浮点数类型、char 和 bool 类型，以及少数其他类型。
    // 所有元素都是 Copy 类型的元组或数组也是 Copy 类型。
    // ==== case1 ====
    // 加上 #[derive(Copy, Clone)]  println!("My label number is: {}", label.number); 就能正常编译通过了
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }

    // 用不是Copy类型的，比如String，不能编译通过
    // #[derive(Copy, Clone)]
    // struct StringLabel { name: String }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let label = Label { number: 123 };
    print(label);
    println!("My label number is: {}", label.number);
}

/// move 操作
fn move_demo() {
    // ==== case1 ====
    let mut v = Vec::new();
    for i in 1001..1006 {
        v.push(i.to_string());
    }

    // let third = v[2]; 错误的, Vec 不允许直接“拿走” move 它里面的元素
    let third = &v[2];
    println!("The third element is {}", third);

    let second = v.remove(1);
    println!("The second element is {}", second);
    println!("The vector is now {:?}", v);

    // ==== case2 ====
    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];

    // for s in v 会 移动所有权
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    // println!("The vector is now {:?}", v); 所以这里不能打印v啦
}

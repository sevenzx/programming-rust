fn main() {
    move_demo();
}

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

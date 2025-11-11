// 因为是 `static mut`，读写都必须在 `unsafe` 块里完成。
static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 1000;

// 'static` 是最长生命周期，活整个程序：要么是编译期就存在的常量/字符串字面量，要么是被 `Box::leak`、`lazy_static!` 等“泄漏”出来的数据；拿到 `&'static T` 就保证程序退出前引用一直有效。
// 'static引用的用途：只有拿到编译期就存在的值（字面量、const 等）才能安全地放进全局静态引用里。
fn store_static_ref(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// 这可以被更简洁地写为：fn pass_ref_to_fn(p: &i32)
// 但现在让我们写出生命周期
fn pass_ref_to_fn<'a>(p: &'a i32) {
    println!("Got a ref to {}", p);
}

fn main() {
    // 1. 引用作为函数参数
    store_static_ref(&32);
    store_static_ref(&WORTH_POINTING_AT);
    // 这样会编译失败：引用 &x 不能生存的比 x 更久，但传递给 store_static_ref 时
    // 我们约束它至少要 和'static 生存的一样久。
    // let x = 64;
    // store_static_ref(&x);

    // 2. 向函数传递引用
    let x = 64;
    pass_ref_to_fn(&x);

    // 3. 返回引用
    // let s;
    // {
    //     let parabola = [9, 4, 1, 0, 1, 4, 9];
    //     s = smallest(&parabola);
    // }
    // assert_eq!(*s, 0);
    // 上面的写法是错误的
    // parabola does not live long enough
    // borrowed value does not live long enough (rustc E0597)
    // 通过 smallest 的签名，我们能看出它的参数和返回值必须有相同的生命周期'a。在我们的
    // 调用中，参数&parabola 不能比 parabola 生存的久，然而smallest 的返回值又至少要和
    // s 生存的一样久。不存在生命周期'a 可以同时满足这两个约束，因此 Rust拒绝了代码

    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0);

    // 4. 结构体包含引用
    structs_contain_references();

    // 5. 不同的生命周期参数
    different_lifetime_params();

    // 6. 忽略生命周期参数
    omit_lifetime_params();
}

// fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }
fn smallest(v: &[i32]) -> &i32 {
    let mut x = &v[0];
    for ele in &v[1..] {
        if *ele < *x {
            x = ele;
        }
    }
    x
}

fn structs_contain_references() {
    // 可以这么理解：`'a` 只是一个“共同的约束标签”，告诉编译器：`S<'a>` 里的引用 `r: &'a i32` 和这个结构本身需要共享同样的生命周期界限。
    // 它不表示谁把生命周期“给”谁，而是说明它们必须一起活在同一段 `'a` 时间里——`S` 不能比 `r` 更长寿，`r` 也不会因为放进 `S` 而延长。
    struct S<'a> {
        r: &'a i32,
    }
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }

    // 这里，我们给了 D 自己的生命周期参数并传递给 S：
    #[allow(dead_code)]
    struct D<'a> {
        s: S<'a>,
    }
}

fn different_lifetime_params() {
    // struct S<'a> {
    //     x: &'a i32,
    //     y: &'a i32,
    // }

    #[allow(dead_code)]
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
    // `r` 可以比 `s` 活得长，只要它指向的 `x` 还在就行
    println!("{}", r);
}

// 总结一句话：
// 单生命周期 'a → 所有字段必须同寿（被最短的约束）。
// 双生命周期 'a, 'b → 字段独立，引用可分离逃逸。

// 情况1 单周期
// main ───────────────────────────────────────────────┐
//   x: i32 (outer)                                    │
//   r: &i32                                            │
//   {                                                  │
//     y: i32 (inner) ───────────────┐                  │
//     {                             │                  │
//       s: S<'a> {                  │                  │
//            x: &x (outer) ---------┘                  │
//            y: &y (inner) --------------┐              │
//       }                                │              │
//       r = s.x                          │              │
//     }  // s 被 drop                    │              │
//   }  // ⚠️ y 被 drop ───────────────────┘             │
// println!("{}", r); // ❌ 报错：'a = y 的生命周期        │
// └────────────────────────────────────────────────────┘

// 情况2 双周期
// main ───────────────────────────────────────────────┐
//   x: i32 ────────────────────────────────┐           │
//   r: &i32                                │           │
//   {                                      │           │
//     y: i32 ───────────────┐              │           │
//     {                     │              │           │
//       s: S<'a, 'b> {      │              │           │
//            x: &'a x  -----┘              │           │
//            y: &'b y  --------------┐     │           │
//       }                            │     │           │
//       r = s.x   // ✅ 'a 可以比 'b 长   │           │
//     }  // s 被 drop                  │               │
//   }  // y 被 drop                    │               │
// println!("{}", r); // ✅ OK ('a = x 的生命周期)       │
// └────────────────────────────────────────────────────┘

fn omit_lifetime_params() {
    // 显而易见的生命期参数Rust能够推导出来的都不用写
    // 如果你的函数是某个类型的方法，并通过引用获取 self 参数，那么会打破这个限制：
    // Rust会假设是返回值中所有内容的生命周期都和 self 的生命周期相同。（Rust中的 self 参数
    // 代表调用这个方法的值，等价于 C++、Java、JavaScript中的 this，或者 Python 中的 self。
    // 我们将在使用 impl 定义方法一节中介绍方法。）

    #[allow(dead_code)]
    struct StringTable {
        elements: Vec<String>,
    }
    #[allow(dead_code)]
    impl StringTable {
        fn find_by_prefi(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    // find_by_prefix 方法的签名是如下签名的缩写：
    // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>
    // Rust假设如果你要借用，那么你会从 self 借用。
}

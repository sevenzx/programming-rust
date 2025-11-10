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
}

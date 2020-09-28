fn main() {
    let test = "test";
    println!("output {}", test); // ! 代表宏, Rust 中的宏比 C 中的强大, C 主要就是展开.

    let i1 = 5; // 相当于 C++ 11 中的: auto i1 = 5;
    let i2: i32 = 5;
    assert_eq!(i1, i2);

    let i3: u32 = 5;
    // assert_eq!(i2, i3); // 类型也需要相等.

    let mut a: f64 = 1.0; // 可变绑定变量.
    let b = 2.0f32; // 值类型显式标记, value + type.

    a = 2.0; // 可变.
    println!("{} {:?}", a, a); // 2 2.0
    println!("{} {:?}", b, b); // 2 2.0
    // {} {:?} 类似 __str__ __repr__ 前者面向用户, 表意即可, 后者面向开发, 力求准确.

    // b = 3.0; // 不可变, 报错.

    let a = a; // 重新绑定为不可变.
    // a = 3.0; // 报错.

    // let -> 绑定表达式～
    let (a, mut b): (bool, bool) = (true, false);
    b = true;
    assert_eq!(a, b); // 可变 和 不可变 可相等.

    const LOCAL_X: i32 = 1; // 常量必须声明类型.
    dbg!(LOCAL_X); // 打印 info, 携更多信息.
    dbg!(GLOBAL_X);

    unsafe {
        dbg!(GLOBAL_X_S);
    }
}

const GLOBAL_X: i32 = 2;

static mut GLOBAL_X_S: i32 = 1; // 多线程下不安全, 限定在 unsafe block 中调用.

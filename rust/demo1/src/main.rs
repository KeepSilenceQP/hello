// cargo clean
// cargo build
// cargo run

// fn main() { // 入口方法.
//     println!("Hello World!");

//     say_hi();
// }

// fn say_hi() {
//     println!("hi");
// }

fn main() {
    // let test = "test";
    // println!("output {}", test);

    // let i1 = 5;
    // let i2: i32 = 5;
    // assert_eq!(i1, i2);

    // let i3: u32 = 5;
    // assert_eq!(i2, i3); // 类型也需要相等.

    // let mut a: f64 = 1.0; // 可变绑定变量.
    // let b = 2.0f32; // 值类型显式标记, value + type.

    // a = 2.0; // 可变.
    // println!("{:?}", a);

    // b = 3.0; // 不可变.

    // let a = a; // 重新绑定为不可变.
    // a = 2.0;

    // let 绑定表达式.
    // let (a, mut b): (bool, bool) = (true, false);
    // b = true;
    // assert_eq!(a, b); // 可变 和 不可变 可相等.

    // let x = 5;
    // let y = if x == 5 { 10 } else { 20 }; // if else 是表达式, 不是语句, 所以有返回值.
    // print!("y: {}", y);

    // let x = Some(5);
    // if let Some(y) = x {
    //     println!("{}", y);
    // }

    // let pair = (0, -2);
    // println!("let me see see {:?}", pair);
    // // 一个 hin 高端的概念 -> 解构.
    // match pair { // 模式匹配.
    //     (0, y) => println!("second: {}", y),
    //     _ => println!("no")
    // }

    // // 若 let 将 pair 解构成了 (0, y), 则执行 {}.
    // if let (0, y) = pair {
    //     println!("y: {}", y);
    // }

    const LOCAL_X: i32 = 1; // 常量必须声明类型.
    dbg!(LOCAL_X); // 打印 info, 携更多信息.
    dbg!(GLOBAL_X);

    unsafe {
        dbg!(GLOBAL_X_S);
    }
}

const GLOBAL_X: i32 = 2;

static mut GLOBAL_X_S: i32 = 1;

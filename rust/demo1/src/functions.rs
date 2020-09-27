fn main() {
    // let z = max(10, 20);
    let z = newMax(10, 20);
    println!("{}", z);
    // 注意区分 expression 与 statement, 表达式(无分号)都有返回值.
    let y = println!("expression ret val."); // 作为整体, 无返回值.
    // 但是, println!("expression ret val.") 有返回值 -> 空元组.
    println!("{:?}", y);
    // Rust 是一门基于表达式的语言, 除了声明语句与表达式语句(以分号结尾的表达式), 都是表达式.
    // 基于表达式是函数式语言的一个重要特征, 表达式总是返回值.
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        // return a;
        a
    } else {
        b
    }
    // return b;
    // b // error, if-else 作为表达式每个分支返回值的类型需要一致, 同时也是整体的返回值类型.
         // 直接这样写, 上面缺少了 else 分支返回值, 用空元组类型补齐, 导致返回类型不一致问题.
}

fn newMax(a: i32, b: i32) -> i32 {
    // return if a > b { a } else { b };
    if a > b { a } else { b }
    // if a > b a else b // error, 必须有 block.
}

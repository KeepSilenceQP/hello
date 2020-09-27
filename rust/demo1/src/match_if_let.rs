fn main() {
    let pair = (0, -2);
    println!("let me see see {:?}", pair);
    // 一个 hin 高端的概念 -> 解构.
    match pair { // 模式匹配.
        (0, y) => println!("second: {}", y),
        _ => println!("no")
    }

    // 若 let 将 pair 解构成了 (0, y), 则执行 {}.
    if let (0, y) = pair {
        println!("y: {}", y);
    }

    let x = Some(5);
    if let Some(y) = x {
        println!("{}", y);
    }
}

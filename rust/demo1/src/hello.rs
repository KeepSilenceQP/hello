/// cargo 基本命令:
/// 
/// - cargo clean
/// - cargo build
/// - cargo run
/// 
/// rust 中有多种注释, 这种是 doc comment.
/// 
/// 使用命令: 
/// 
/// >rustdoc src/hello.rs 
/// 
/// 即可生成.
/// 
/// 和其他语言一样, **main** 方法为入口方法.
pub fn main() {
    println!("Hello World!");

    say_hi();
}

/// **pub** 的 fn 的文档注释才会写入 doc 中.
pub fn say_hi() {
    println!("hi");
}

fn main() {
    let mut x = [1, 2, 3];
    // for i in &x {
    for i in 0..x.len() {
        // print!("{} ", i);
        print!("{} ", x[i]);
    }
    println!();

    let mut v = 100;
    loop {
        print!("{} ", v);
        v = v - 1;
        if v < 50 {
            println!("break from loop!!!");
            break;
        }
    }

    // 循环标签.
    let mut arr = [[1, 2, 3], [4, 5, 6], [6, 7, 8]]; // 此种定义形式, 若少一列, 报错:
    // expected an array with a fixed size of 3 elements, found one with 2 elements
    'outer: for subArr in &arr {
        'inner: for i in subArr {
            println!("{}", &i);
            // if (&7 == i) { // Rust 不喜欢括号!!!
            if &7 == i {
                println!("find 7!!!");
                break 'outer;
            }
        }
    }
}

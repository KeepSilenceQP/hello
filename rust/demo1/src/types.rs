fn main() {
    #[derive(Debug)] // 直接使用编译器的 `Debug` trait, 否则无法使用 {:?} 打印.
    struct Student {
        name: String,
        age: i32,
        male: bool
    }

    let mut stu = Student { // 需整体定义 mut, 定义单个无效.
        name: String::from("qinpeng"),
        age: 22,
        male: true
        // mut male: true // expected identifier, found keyword
    };

    println!("{:?}", stu);
    // dbg!(stu); // move occurs because `stu` has type `Student`, which does not implement the `Copy` trait

    stu.age = 23;

    println!("{:#?}", stu);

    #[derive(Debug)]
    enum Gender {
        MALE,
        FEMALE
    }

    #[derive(Debug)]
    struct NewStudent {
        name: String,
        age: i32,
        male: Gender
    }

    let stu1 = NewStudent {
        name: "qinpeng".to_string(),
        age: 25,
        male: Gender::MALE // 可嵌套.
    };

    println!("{:?}", stu1);

    enum MyMoney {
        Value(i32),
        None
    }

    let myMoney = MyMoney::Value(100); // 域分隔符.

    match myMoney {
        MyMoney::Value(v) => {
            println!("v: {}", v);
        }
        MyMoney::None => {
            println!("wow! must cover!");
        }
    };
}

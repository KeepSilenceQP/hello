fn main() {
    #[derive(Debug)] // 直接使用编译器的 `Debug` trait.
    struct Student {
        name: String,
        age: i32,
        male: bool
    }

    let mut stu = Student { // 整体定义 mut
        name: String::from("qinpeng"),
        age: 22,
        male: true
    };

    println!("{:?}", stu);

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
        male: Gender::MALE
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

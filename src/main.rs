/***********************
 *  泛型(generic)，特性(trait)  生命周期
 ***********************/
use std::cmp::PartialOrd;
use std::fmt::Display;

fn main() {
    // ! 重复代码
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for num in &number_list {
        if num > &largest {
            largest = *num;
        }
    }

    let number_list2 = vec![34, 50, 25, 10, 65];
    let mut largest2 = number_list2[0];
    for num in &number_list2 {
        if num > &largest2 {
            largest2 = *num;
        }
    }
    // * 解决重复
    fn largest_fn(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let max_list = largest_fn(&number_list);
    let max_list2 = largest_fn(&number_list2);
    println!("max_list:{}  |  max_list2:{}", max_list, max_list2);

    // todo generic 泛型
    /*
     * 提高代码的"复用"能力
     * 编译器会在编译时把泛型的占位符（T）替换成具体的类型，这个叫 “单态化”，泛型不影响性能
     * fn largest<T>(list: &[T]) -> T {...}
     */
    // ? 如果参数过多就需要重构泛型

    // ! 思路是这样，类型会报错
    //  fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    //     let mut largest = list[0];
    //     for item in list {
    //         if item > &largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }

    // TODO struct 结构体泛型
    struct Point<T> {
        bx: T,
        by: T,
    }

    let p = Point { bx: 10, by: 10 };
    println!("{},{}", p.bx, p.by);

    struct Point2<T, U> {
        bx: T,
        by: U,
    }
    let p = Point2 { bx: 10, by: 10.01 };
    println!("{},{}", p.bx, p.by);

    // TODO enum 枚举泛型
    /*
     * 可以让枚举变体持有泛型数据类型，例如：Option<T>, Result<T, E>
     */
    enum ResultGoods<T, E> {
        Ok(T),
        Err(E),
    }

    // TODO 方法泛型
    impl<T> Point<T> {
        fn run_x(&self) -> &T {
            &self.bx
        }
    }

    impl Point<u64> {
        fn string_and_bool(&self) -> u64 {
            self.bx
        }
    }

    let num: i32 = 100;
    let _ = Point { bx: num, by: 10 };
    // ! px.string_and_bool();  这里会报错，因为只有T类型为u64才能使用这个方法
    // 结构体里方法名可以和结构体里面的方法类型不同
    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, outher: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                bx: self.bx,
                by: outher.by,
            }
        }
    }

    let p1 = Point2 { bx: 10, by: 10.01 };
    let p2 = Point2 {
        bx: 10.02,
        by: "world",
    };
    let p3 = p1.mixup(p2);
    println!("{},{}", p3.bx, p3.by);

    // TODO trait 特性
    /*
     * 与interface有点类型
     * trait告诉编译器某种类型具有哪些并且可以与哪些类型共享功能
     * 以抽象的方式定义共享行为
     * trait bounds（约束）： 泛型类型参数置顶为实现了特定行为的类型
     */

    // 定义 trait
    pub trait Summary {
        // 未声明实现
        fn summarize(&self) -> String;

        // 默认实现
        fn summarize2(&self) -> String {
            String::from("Summer Rust")
        }
        // 默认实现(重载用)
        fn summarize3(&self) -> String {
            // 但必须summarize需要被实现
            format!("{}, {}", String::from("Summer Rust"), self.summarize())
        }
    }

    // 实现 trait
    struct Book {
        title: String,
        price: f32,
    }

    impl Summary for Book {
        fn summarize(&self) -> String {
            println!("title:{}, price:{}", &self.title, &self.price);
            format!("{}", self.title)
        }
    }
    let book = Book {
        title: String::from("Rust Book"),
        price: 10.32,
    };
    println!("{}", book.summarize());
    // 执行默认实现
    println!("{}", book.summarize2());

    println!(": {}", book.summarize3());

    // 对 trait 的默认实现重载实现
    impl Book {
        fn summarize3(&self) -> String {
            format!("overload summarize 3")
        }
    }
    println!("{}", book.summarize3());

    // trait 约束
    // ? 无法为外部的类型来实现外部的trait，保证一致性（孤儿原则）

    // TODO trait 作为参数

    // 参数入的 item 必须实现了 Summary
    fn notify(item: impl Summary) -> String {
        item.summarize();
        format!("notify")
    }

    // trait bound语法，解决复杂情况
    fn notify2<T: Summary>(item: T) -> String {
        format!("notify2")
    }

    // 使用 + 指定多个 trait bound
    fn notify3<T: Summary + Display>(item: T) -> String {
        format!("notify2")
    }

    // 使用 + 指定超多 trait bound
    fn notify4<T, U>(item: T, item2: U) -> String
    where
        T: Summary + Display,
        U: Display,
        //....
    {
        format!("notify2")
    }

    // TODO trait 作为返回值
    // * 只能返回一个类型
    fn notify5(item: impl Summary) -> impl Summary {
        item
    }

    // ? 实现泛型中的错误
    /*
     * 栈内存实现了Copy trait
     * 堆内存实现了Clone trait
     */
    fn largest_generic2<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 传入String类型的改造
    fn largest_generic3<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();
        for item in list.iter() {
            if item > &largest {
                largest = item.clone();
            }
        }
        largest
    }
    // OR 直接引用list
    fn largest_generic4<T: PartialOrd + Clone>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // TODO 有条件实现 trait

    struct Pair<T> {
        x: T,
        y: T,
    }

    // 所有Pair都默认实现new函数
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // 所有 T 包含 Display + PartialOrd都可以访问 cmp_display 函数
    impl<T> Pair<T>
    where
        T: Display + PartialOrd,
    {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    /*
     * 为所有 T 满足Display特性的都实现了ToString方法
     * impl<T: fmt::Display> ToString for T {}
     * 例如：3.to_string();
     */

    println!("Hello, world!");

    /***************
     *  生命周期    *
     ***************/
    /*
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }
    */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // ! fn longest2<'a>(x: &'a str, y: &str) -> &'a str{
    fn longest2<'a>(x: &'a str, y: &str) -> String {
        let result = String::from("abc");
        // ! result.as_str()
        result
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result);

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years age..");
    let first_sentence = novel.split(".").next().expect("Could not found a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // 生命周期的省略,编译器对可忽略的生命周期进行省略
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..];
            }
        }
        &s[..]
    }

    // 方法定义中的生命周期。
    struct ImportantExcerpt2<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt2<'a> {
        fn level(&self) -> i32 {
            3
        }
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            self.part
        }
    }

    // 静态声明周期
    let s: &'static str = "i have a static lifetime";

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

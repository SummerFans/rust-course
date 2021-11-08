use std::panic;

fn main() {
    let cool = '🤣';
    println!("基础教程开始啦！ {}", cool);

    // TODO 不可以给不可变变量赋值
    println!("------[不可以给不可变变量赋值]--------------------------");
    let num = 100;
    println!("num is: {}", num);
    // ! num = 6;  > 变量是不可变变量，如果也要可变要申明 mut
    // * Suuess
    let mut num_success = 100;
    println!("num_success is {}", num_success);
    num_success = 20;
    println!("update num_success is {}", num_success);

    // TODO 常量
    println!("------[常量]--------------------------");
    /*
     * 不可使用mut，常量永不可变
     * 可在任何作域声明常量，包括全局
     * 常量不可在运行期间计算出值
     */
    const MY_IS_COUNT: u32 = 100_000;
    println!("COUNT IS {}", MY_IS_COUNT);

    // TODO Shadowing (覆盖) New
    println!("------[Shadowing (覆盖) ⭐️]--------------------");
    /*
     * 主要是将不同类型变量但想使用相同名称的一种方式
     * 常见的例子：输入整数字符，但类型是String，想转换成Int类型可变量名称不变
     */
    let mut num = String::new();
    num.push_str("1000");
    println!("{} is String", num.as_str());

    // ? Shadowing触发点
    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            panic!("err: {}", err);
        }
    };
    println!("{} is i32", num);

    // TODO 复合类型
    println!("------[复合类型]------------------------");
    /*
     * 复合类型有两种：元组（Tuple），数组
     * Tuple:多类型数据放在一个类型里，长度是固定，申明后无法改变
     * 数组: 类型必须相同，且声明后长度固定
     */

    let tup: (i32, f64, char) = (500, 30.32, '🥰');
    println!("i32:{} | f64:{} | u8:{}", tup.0, tup.1, tup.2);
    // or
    let (x, y, z) = tup;
    println!("i32:{} | f64:{} | u8:{}", x, y, z);

    let arr: [i32; 5] = [10, 2, 3, 4, 5];

    for ele in arr {
        println!("value: {}", ele);
    }
    // 取索引值
    for (index, ele) in arr.iter().enumerate() {
        println!("{} | {}", index, ele)
    }

    // TODO 函数
    println!("------[函数]------------------------");
    /*
     * 函数名使用下划线区分单词
     * 调用函数时参数叫: arguments, 定义函数参数时称做：parameters
     */
    fn another_function(x: i32) {
        // -> parameter
        println!("{}", x)
    }
    another_function(10); // --> argument
    let fun_num = {
        // ! 10+6; -> 因为分号结束代表表达式结束,非函数结束，所以不允许。除非：return 10+6;
        10 + 6
    };
    println!("{}", fun_num);
    // 单返回值函数
    fn five(x: i32) -> i32 {
        x + 5
    }
    // 多返回值函数
    fn fives(x: i32) -> (i32, i64) {
        let num: i64 = 5;
        (x, num)
    }
    let five = five(10);
    println!("{}", five);

    let (vi32, vi64) = fives(5);
    println!("i32:{} and i64:{}", vi32, vi64);

    // TODO 控制流
    println!("------[控制流]------------------------");
    println!("------[IF 语句]------------------------");
    /*
     *  if语句判断必须是bool值
     */
    if vi32 == five {
        println!("five is {}", five)
    } else {
        println!("five not {}, is {}", five, vi32)
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }

    println!("------[⭐️ match 语句⭐️ ]------------------------");
    /*
     * 允许一个值与一系列模式进行匹配，并执行匹配的模式对应代码
     * 模式可以是字面值、变量名，通配符
     * 必须穷举所有的可行性
     */
    #[derive(Debug)]
    enum UsState {
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // 绑定模式匹配
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // ? {:?} 这是因为, println! 宏默认使用类型的Display格式输出,
                // ? 意思是打印出来的输出, 之前所有的基本类型都实现了Display, 但是因为结构体
                // ? rust并不知道你想要输出什么, 所以没有提供Display实现
                // * 需要加 #[derive(Debug)]
                println!("state quarter from {:?}!", state);
                25
            }
        }
    }
    println!(
        "Nickel:{} | Quarter:{} | Dime:{} | Penny:{}",
        value_in_cents(Coin::Nickel),
        value_in_cents(Coin::Quarter(UsState::Alaska)),
        value_in_cents(Coin::Dime),
        value_in_cents(Coin::Penny)
    );

    // * ⭐️  OPTION 枚举
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None, // ! > 必须穷尽所有的可能
        }
    }

    let d = plus_one(Some(100));
    println!("{}", d.unwrap());

    //* 忽略返回值
    let v = 10u8;
    match v {
        1 => println!("match one"),
        3 => println!("match three"),
        5 => println!("match five"),
        7 => println!("match seven"),
        _ => (),
    }

    // * if let
    if let Some(10) = Some(v) {
        println!("ten");
    }

    println!("------[循环语句]------------------------");
    /*
     * loop是无限循环使用 break 停止循环，
     */
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 递增到10 时 乘上 2
        }
    };
    println!("{}", result);

    /*
     * while 条件循环，每次循环前都判断一次条件  🟡 效率低，不建议
     */
    let mut numbes = 3;
    while numbes != 0 {
        println!("{}!", numbes);

        numbes -= 1;
    }

    println!("LIFTOFF!!!");

    /*
     * for 循环  🟢 建议使用 安全简洁，效率高
     */
    let for_data = [10, 20, 30, 40, 50];
    for d in for_data.iter() {
        println!("the value is: {}", d);
    }

    /*
     * for Range 
     */
    for num in (1..5).rev() {
        println!("range {}", num);
    }
    
}

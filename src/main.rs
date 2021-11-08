use std::panic;

fn main() {
    let cool = '🤣';
    println!("基础教程开始啦！ {}", cool);

    // TODO 不可以给不可变变量赋值
    println!("------[不可以给不可变变量赋值]--------------------------");
    let num = 100;
    println!("num is: {}", num);
    // !num = 6;
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
    println!("------[Shadowing (覆盖) New]--------------------");
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

    let arr:[i32;5] = [10,2,3,4,5];

    for ele in arr {
        println!("value: {}", ele);
    }
    // 取索引值
    for (index, ele) in arr.iter().enumerate() {
        println!("{} | {}", index, ele)
    }
}

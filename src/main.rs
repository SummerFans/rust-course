use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("请输入你猜测的数字！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("err: {}",err);
                continue;
            }
        };

        // println!("随机数：{}", secret_number);

        // println!("你输入的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }
}

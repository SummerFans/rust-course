use std::fs::File;
use std::io::{self,ErrorKind, Read};

/************************
 *     TODO 异常处理     *
 ************************/
/*
 * rust 错误处理概述
 * 1.可恢复错误  Result<T,E>
 * 2.不可恢复错误 panic!
 * 
 * 原则
 * 1.定义可能失败的函数，优先考虑返回Result
 * 2.不可恢复使用panic
 * 当代码处于损坏状态(Bad state)，某些假设、保证、约定、被打破或不可变性被打破
 *   - 演示某些概念：unwrap
 *   - 原型代码： unwrap、expect
 *   - 测试：unwrap,expect
 */

fn main() {

    // todo panic
    /*
     * panic! 宏执行流程
     * 1.打印一个程序错误信息
     * 2.展开unwind、清理调用栈(stack)
     * 3.退出程序
     */
    // ? 为对应panic，展开或中止(abort)调用栈: abort是立即终止调用栈，将内存处理交给系统清理（效率高）
    // * Cargo.toml 修改 abort 后可让二进制包变的更小

    // * 环境变量中添加 “RUST_BACKTRACE=1” 可以显示完整的栈信息 (寻找是我们写的panic报错，还是其他依赖包的报错信息);

    // ! 会终止程序 panic!("error")


    // todo Result<T,E> 枚举类型
    /*
     * enum Result<T, E> {
     *      Ok(T),
     *      Err(E),
     * }
     */
    let f = File::open("hello.txt");
    
    let _ = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error Createing file: {:?}", e),
            }
            outher_error => panic!("Error Opening the file {:?}", outher_error),
        }
    };

    let _ = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("{}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    println!("Hello world");

    // todo 传播错误
    /*
     * ? 运算符返回整个函数返回值Error
     * ? 与 from(std::convert::From) 函数 用于错误类型转换，?操作符会被隐式from函数处理
     * 适用于不同错误原因转换返回同一种错误类型，只要每个错误类型实现了转化为所返回错误类型的from函数
     */
    // ? 运算符只能返回 Result， Option 或者实现了try的类型
    fn read_username_from_file() -> Result<String, io::Error> {
        let f  =File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // 最简单方式
    fn read_username_from_file_fast() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    println!("{:?}", read_username_from_file());
    println!("{:?}", read_username_from_file_fast());
}

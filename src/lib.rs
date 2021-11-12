/***************************
 *         单元测试         *
 **************************/
/*
 * cargo test 模式行为是并行测试所有测试，捕获（不显示）所有输出
 *  cargo test --help       // cargo test 帮助参数
 *  cargo test -- --help    // 可以用在 -- 之后的所有参数
 *  @param
 *  --test-threads={int}    //使用线程数量
 *  --show-output           // 输出成功的输出(println!)
 * 
 *
 * 标注属性（attribute）
 * #[cfg(test)]     
 * #[test]          测试函数标注
 * #[should_panic]  应该恐慌
 *
 * 函数体执行执行三个操作
 * 1.准备数据/状态
 * 2.运行被测试的代码
 * 3.断言（Assert）结果
 *     - assert! 宏接收bool，如果true测试成功，测试失败出发panic
 *     - assert_eq! 相等宏（会打印失败值）
 *     - assert_ne! 不等于宏（会打印失败值）
 */

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, outher: &Rectangle) -> bool {
        self.length > outher.length && self.width > outher.width
    }

    pub fn print_test(&self, num: u32) -> u32 {
        self.width + num
    }
}

pub struct Guess {
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater tan of equal to 1 got {}.",
                value
            );
        } else if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn result_value(&self, num: u32) -> Result<u32, String> {
        if self.value > num {
            Err(String::from("err: num less than value"))
        } else {
            Ok(self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    // ! 导入所有引用
    use super::*;

    #[test]
    fn is_works() {
        assert_eq!(2, 2);
    }
    #[test]
    fn anouther() {
        // ! assert_eq!(3, 2); 触发panic，处理失败
        assert_ne!(3, 2);
    }

    #[test]
    fn test_can_hold() {
        let sarger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&sarger))
    }

    #[test]
    fn test_print_test() {
        let sarger = Rectangle {
            length: 8,
            width: 7,
        };
        assert!(
            sarger.print_test(10) == 17,
            "不相等,width:值是{}",
            sarger.width
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn test_guess() {
        Guess::new(200);
    }

    #[test]
    #[ignore]  
    fn test_result_value() -> Result<(), String> {
       if 10 + 20 == 30 {
           Ok(())
       } else {
           Err(String::from("error"))
       }
    }


    // todo 按照测试名称测试集
    // cargo test test_result_value  // 运行单个测试
    // cargo test test               // 运行以test开头的测试，类似：test*

    // todo 忽略某些测试，运行剩余测试
    // #[ignore]                     // 忽略耗时测试
    // cargo test -- --ignored       // 执行忽略测试

    
    // todo 单元测试
    /*
     * 专注单一模块测试粒度最小，可以对隔离（privale）的模块进行测试
     * 一般都把单元测试和测试的代码放同个文件中，用 #[cfg(test)] 来区分单元测试和普通代码
     * #[cfg(test)] 部分只有cargo test才能执行 cargo build 不会执行
     */

}

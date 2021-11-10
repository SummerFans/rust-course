// TODO Crate
/*
 * 模块系统
 * - Package 包：cargo的特性，构建测试共享crate
 * - Crate 单元包：一个模块树，可以产生一个library或可执行文件
 * - Module 模块：控制代码组织，作用域，私有路径
 * - Path： 为struct、function或module项命名的方式
 */

/*
 * Crate 类型：1.binary（二进制） 2.library(库)
 * Crate Root：源代码文件,rust编译器从这里开始，组成你的Crate的根Module
 *
 * Crate上一层是一个Package，package存在下面部分
 * - 包含一个cargo.toml,它描述了如何构建这些Crates
 * - 只能包含0 or 1个library crate
 * - 可以包含任意数量的binary crate
 * - 必须至少包含一个crate（library or binary）
 */

/*
* Module
* 在一个crate内，将代码分组
* 增加可读性，易于复用
* 控制项目（item）的私有性。public/private
* 使用mod关键字建立module，mod可以嵌套mod，可包含struct， enum，常量， trait，函数等
*/
// ! lib.rs 和 main.rs 都是单独crate模块

// TODO PATH 路径
/*
 * 为了在rust的模块中找到某个条目，需要使用“路径”
 * 路径的两种形式
 *  1.绝对路径： 从crate root开始，使用crate名或字面值crate
 *  2.相对路径： 从当前模块开始，使用self，super或当前模块的标识符
 * 路径至少由一个标识符组成，标识符之前使用::
 *
 */

/**
 *  ⭐️ mod 如果使用分号 “;” 结束代表在src目录下查询模块
 */

fn main() {
    println!("Hello, world!");
}

use rust_course;

mod common;

/******************
 *    集成测试     *
 *****************/
// ! binary crete 不能在目录下创建集成测试
// ? 集成测试覆盖率是一件非常重要的事情
/*
 * crate root下面创建一个tests文件夹，测试时候会自动根据这个目录去寻找测试文件
 * 集成测试主要是为了测试对外部调用接口的测试，测试对外的接口是否满足预期
 * cargo test --test rust_test  // 单独集成测试文件
 */

#[test]
fn is_add_two() {

    let sarger = rust_course::Rectangle {
        length: 8,
        width: 7,
    };
    
    assert!(!common::connect_pgsql().can_hold(&sarger));
}

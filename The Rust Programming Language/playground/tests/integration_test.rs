/**
 * [集成测试]
 * cargo test 函数名/模块名        => 运行指定函数/模块的测试
 * cargo test --test 文件名 => 运行指定测试文件内的所有测试
 */
use playground; // 导入 library crate(和项目同名)

// 集成测试的公共模块 => tests/common/mod.rs
mod common;

#[test]
fn it_works() {
    assert_eq!(playground::datasets::add_one(3), 4);
}

#[test]
fn it_works2() {
    assert_eq!(common::add_one(4), 4);
}

// use super::*;

#[test]
fn test_fn() {
    assert_eq!(2, 1 + 1);
}

#[test]
// 常规模式忽视的测试
// 通过 cargo test -- --ignored 来单独测试
#[ignore]
fn test_fn2() {
    assert_eq!(3, plus_one(2));
}

// 测试私有函数
fn plus_one(x: i32) -> i32 {
    x + 1
}

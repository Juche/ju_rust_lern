//! # format
//!
//! 打印操作由 `std::fmt` 里面所定义的一系列宏来处理:
//! * `format!` 将格式化文本写到字符串
//! * `print!` 与 `format!` 类似, 但将文本输出到控制台(`io::stdout`)
//! * `println!` 与 `print!` 类似, 但输出结果追加一个换行符
//! * `eprint!` 与 `print!` 类似, 但将结果输出到标准错误(`io::stderr`)
//! * `eprintln!` 与 `eprint!` 类似, 但输出结果追加一个换行符
//!

pub fn run() {
    // 占位符 `{}`
    println!("{} days", 31);

    // 位置参数 `{0}, {1}`
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );

    // 指定特殊格式 `:b` 二进制; `:o` 八进制; `:x` 十六进制; `:p` 指针地址
    println!("Binary representation 255: {:b}", 255);
    println!("Octal representation 255: {:o}", 255);
    println!("Hex representation 255: {:x}", 255);
    println!("Pointer 255: {:p}", &255);

    // 指定宽度 `:width`
    println!("{0:>width$}", "hello", width = 10);
    println!("{0:_>width$}", "hello", width = 10);
    println!("{0:_<width$}", "hello", width = 10);

    // 指定精度 `:precision`
    println!("One decimal place: {0} = {1:.1}", "x", 3.14159);
    println!("Two decimal places: {0} = {1:.2}", "x", 3.14159);
    println!("Three decimal places: {1:.*}", 3, 3.14159);
    println!("Three decimal places: {} = {2:.*}", "x", 3, 3.14159);
}

/*
 * 打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：
 *
 * format!：将格式化文本写到字符串。
 * print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
 * println!: 与 print! 类似，但输出结果追加一个换行符。
 * eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
 * eprintln!：与 eprint! 类似，但输出结果追加一个换行符.
 */

pub fn run() {
    // `{}` 占位符
    println!("{} days", 31);
    // `{0}, {1}` 占位符使用位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // 命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );
}

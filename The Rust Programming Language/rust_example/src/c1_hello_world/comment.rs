//! # comment
//!
//! ```
//! 普通注释，其内容将被编译器忽略掉：
//!  // 单行注释，注释内容直到行尾。
//!  /* 块注释，注释内容一直到结束分隔符。 */
//! 文档注释，其内容将被解析成 HTML 帮助文档：
//!  /// 为接下来的项生成帮助文档。
//!  //! 为注释所属于的项（译注：如 crate、模块或函数）生成帮助文档。
//!
//!  常用（文档注释）部分
//!    Examples：这个函数的示例。
//!    Panics：这个函数可能会 panic! 的场景。并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数。
//!    Errors：如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误。
//!    Safety：如果这个函数使用 unsafe 代码，这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）。
//! ```
//! # [DOC]
//! `cargo doc --open` 生成并打开文档
//!
//! `cargo test` 会执行文档注释中的代码部分
//!
//! [注释](https://rustwiki.org/zh-CN/rust-by-example/hello/comment.html)
//!
//! [文档注释](https://rustwiki.org/zh-CN/book/ch14-02-publishing-to-crates-io.html#%E7%BC%96%E5%86%99%E6%9C%89%E7%94%A8%E7%9A%84%E6%96%87%E6%A1%A3%E6%B3%A8%E9%87%8A)

/// 这里给出一个“人”的结构体表示
pub struct Person {
    /// 一个人必须有名字（不管 Juliet 多讨厌她自己的名字）。
    pub name: String,
}

/// 返回具有指定名字的一个人
///
/// # 参数
///
/// * `name` - 字符串切片，代表人的名字
///
/// # 示例
///
/// ```
/// // 在文档注释中，你可以书写代码块
/// // 如果向 `rustdoc` 传递 --test 参数，它还会帮你测试注释文档中的代码！
/// use rust_example::c1_hello_world::comment::Person;
/// let person = Person::new("Juching");
/// person.hello();
///
/// assert_eq!(person.name, "Juching");
/// ```
impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

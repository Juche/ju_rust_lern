//! # custom format
//! 格式化 traits
//! 当请求使用特定类型的参数格式化时，实际上是在请求将参数归因于特定的 trait。 这允许通过 {:x} 格式化多种实际类型 (例如 i8 和 isize)。类型到 traits 的当前映射是：
//! ```
//! nothing ⇒ Display
//! ? ⇒ Debug
//! x? ⇒ Debug 带有小写十六进制整数
//! X? ⇒ Debug 带有大写十六进制整数
//! o ⇒ Octal
//! x ⇒ LowerHex
//! X ⇒ UpperHex
//! p ⇒ Pointer
//! b ⇒ Binary
//! e ⇒ LowerExp
//! E ⇒ UpperExp
//! # => 此标志表示应使用 “alternate” 打印形式(漂亮地打印)
//! ```

use std::fmt;

// println!("{:?}", "hello");
// println!("{:?}", (12, true, "hello"));

#[derive(Debug)]
struct Vec2D {
    x: isize,
    y: isize,
}

impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = ((self.x * self.x) + (self.y * self.y)) as f64;
        let v = v.sqrt();
        write!(f, "({}, {}, {})", self.x, self.y, v)
    }
}

pub fn run() {
    let v = Vec2D { x: 3, y: 4 };

    println!("{v}"); // println!("{}", v);
    println!("{v:?}"); // println!("{:?}", v);
}

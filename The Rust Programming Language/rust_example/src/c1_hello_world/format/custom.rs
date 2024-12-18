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

#[derive(Debug)]
struct Complex {
    real: f64,
    image: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.image)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{i}: {v}")?;
        }
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_ori = if self.lat > 0.0 { "N" } else { "S" };
        let lon_ori = if self.lon > 0.0 { "E" } else { "W" };

        write!(
            f,
            "{}: {:.3}°{}, {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_ori,
            self.lon.abs(),
            lon_ori
        )
    }
}

#[derive(Debug)]
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = format!("{:02X}", self.r);
        let g = format!("{:02X}", self.g);
        let b = format!("{:02X}", self.b);
        write!(
            f,
            "RGB ({0}, {1}, {2}) => #{r}{g}{b}",
            self.r, self.g, self.b
        )
    }
}

pub fn run() {
    let v = Vec2D { x: 3, y: 4 };
    println!("{v}"); // println!("{}", v);
    println!("{v:?}"); // println!("{:?}", v);

    let c = Complex {
        real: 3.3,
        image: 7.2,
    };
    println!("{c}");
    println!("{c:?}");

    let l = List(vec![1, 2, 3]);
    println!("{l}");

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            r: 128,
            g: 255,
            b: 90,
        },
        Color { r: 0, g: 3, b: 254 },
    ]
    .iter()
    {
        println!("{}", *color);
        println!("{:?}", *color);
    }
}

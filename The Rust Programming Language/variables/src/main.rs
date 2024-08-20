/**
 * Rust 数据类型
 * 整数类型: i8、i16、i32、i64、i128、isize
 * 整数类型(无符号): u8、u16、u32、u64、u128、usize
 * 浮点类型: f32、f64
 * 布尔类型: bool
 * 字符类型: char
 * 字符串类型: &str
 * 元组类型: tuple
 * 数组类型: array
 * vector
 */
fn main() {
    println!("Hello, world!");

    let mut x = 5;
    x += 1;
    println!("x = {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    let x = 66; // shadowing(可声明同名变量,后续变量会隐藏前面的变量)
    println!("x = {}", x);

    // u8 的取值范围是 0-255, 超出范围会报错
    // let u8_max: u8 = 256;
    // println!("u8_max = {}", u8_max);

    // 字符类型使用单引号,只能有一个字符
    let chr: char = 'a';
    println!("chr = {}", chr);

    // 字符串类型使用双引号
    const CN: &str = "🇨🇳";
    println!("CN = {}", CN);
    const AR: &str = "🇦🇷";
    println!("AR = {}", AR);

    // Tuple 元组,元组的长度是固定的
    // 元组中的元素可以是不同类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    // 元组解构
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    // 访问元组元素，使用 `. + 索引` 来访问
    println!("tup.0 = {}", tup.0);

    // Array 数组,数组的长度是固定的
    // 数组数据类型必须相同
    // 数组存储在栈中
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
    let arr = [3; 5]; // [3, 3, 3, 3, 3]
    println!("arr = {:?}", arr);

    let mon = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    println!("mon = {:?}", mon);
    let m2 = mon[1];
    println!("m2 = {}", m2);

    // Vector
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    println!("v = {:?}", v);
}

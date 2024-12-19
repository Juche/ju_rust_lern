pub fn run() {
    let bin = 0b1010_1010;
    let oct = 0o777;
    let hex = 0xff;
    println!("bin = {bin}, oct = {oct}, hex = {hex}");

    // 加减乘除运算
    let sum = 5u32 + 6;
    // let diff = 1u32 - 2; // attempt to compute `1_u32 - 2_u32`, which would overflow
    let diff = 1.2 - 2.3f32;
    let mul = 2 * 3;
    let div = 2f64 / 3f64;
    println!("sum = {sum}, diff = {diff}, mul = {mul}, div = {div}");

    // 逻辑运算
    let t = true;
    let f = false;
    println!("t && f = {}", t && f);
    println!("t || f = {}", t || f);
    println!("!t = {}", !t);
    println!("!f = {}", !f);

    // 位运算
    println!("1 << 2 = {}", 1 << 2);
    println!("1 >> 2 = {}", 1 >> 2);
    println!("1 & 2 = {}", 1 & 2);
    println!("1 | 2 = {}", 1 | 2);
    println!("1 ^ 2 = {}", 1 ^ 2);
}

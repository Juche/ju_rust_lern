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
    // let mut v: Vec<i32> = Vec::new(); // 通过 new 方法创建
    let mut v = vec![1, 2, 3, 4, 5]; // 通过 vec! 宏创建
    v.push(6); // 添加元素
    println!("v = {:?}", v);

    let v_6 = v.get(6); // 通过 get 访问元素 => 超出范围不会引发程序崩溃
    match v_6 {
        Some(val) => println!("v_6 = {}", val),
        None => println!("There is no v_6 val!"),
    }
    // let v_6 = &v[6]; // 通过索引访问元素 => 超出范围会 panic 引发程序崩溃
    // println!("v_6 = {}", v_6);

    // String
    let mut s1 = String::new();
    let s2 = "abc".to_string();
    let s3 = String::from("abc");

    // push_str 方法将字符串切片附加到 String
    s1.push_str(&s2);
    s1.push_str("def");

    // push 方法把单个字符附加到 String
    s1.push('g');

    println!("s1 = {}", s1);

    // + 连接字符串 `String + &str`, 会获取第一个参数的所有权
    let s4 = s2 + &s3;
    println!("s4 = {}", s4);

    // format! 宏拼接字符串,更灵活,不会获取任何参数的所有权
    let s5 = String::from("ABC");
    let s6 = String::from("DEF");
    let s7 = String::from("GHI");
    // let s8 = s5 + "-" + &s6 + "-" + &s7;
    let s8 = format!("{}-{}-{}", s5, s6, s7);
    println!("s8 = {}", s8);

    // len() 方法获取字节长度
    let s1_len = s1.len();

    // String 不支持索引形式获取字符 &s1[0]
    // 因为不同编码规范的字符对应的 Unicode 标量值不一致
    // 字节(Bytes) => bytes() 方法
    // 标量值(Scalar Values) => chars() 方法
    // 字形簇(Grapheme Cluster) => 最接近 `字母` 的概念, 标准库未提供对应的方法,可寻找对应的第三方库处理

    // 切割 String, 可通过 [] 和一个范围来创建字符串的切片(包含起始索引,不包含终止索引)
    // 需谨慎使用,沿字符边界切割(某些语言一个字符包含多个字节)
    let s9 = &s1[0..2];

    // HashMap
    use std::collections::HashMap; // 不在 prelude 模块中,需手动导入

    // let mut scores: HashMap<String, i32> = HashMap::new();
    let mut scores = HashMap::new(); // 创建
    scores.insert(String::from("Blue"), 10); // 添加数据
    scores.insert(String::from("Yellow"), 50); // 添加数据

    // collect 方法可以将数据整合成多种数据类型, 包括 HashMap (返回值需显式指明类型)
    // 将 teams 和 init_score 遍历项缝合组成元组数组, 然后通过 collect 创建指定的 HashMap 类型
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_score = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(init_score.iter()).collect();

    // get 方法访问 HashMap 中的值, 参数: K, 返回 Option<&V>
    // let blue_score = scores.get(&String::from("blue"));
    let blue_score = scores.get(&String::from("Blue"));
    match blue_score {
        Some(val) => println!("blue_score = {}", val),
        None => println!("blue_score not found!"),
    }

    // for 循环遍历 HashMap
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // 更新 HashMap
    // 新增值/使用新值替换现有值 `HashMap.insert(K, V)`
    let mut person = HashMap::new();
    person.insert(String::from("name"), String::from("Juching"));
    person.insert(String::from("name"), String::from("Juche"));
    println!("person = {:#?}", person);

    // 保留现有值,忽略新值 `HashMap.entry(K).or_insert(V)`
    let new_age = person
        .entry(String::from("age"))
        .or_insert("18".to_string());
    println!("new_age = {}", new_age);
    let new_name = person
        .entry(String::from("name"))
        .or_insert(String::from("JU"));
    println!("new_name = {}", new_name);
    println!("person = {:#?}", person);

    // 合并新值和现有值
    let text = "hello world wonderful world !";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("count = {}", count);
    }

    println!("map = {:#?}", map);
}

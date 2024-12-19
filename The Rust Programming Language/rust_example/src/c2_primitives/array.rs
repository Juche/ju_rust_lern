pub fn run() {
    // 定长数组
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5]; // [3, 3, 3, 3, 3]

    // `len` 获取数组长度
    println!("a1 len: {}", a1.len());
    // 下标从 0 开始访问
    println!("a2[0]: {}", a2[0]);
    // std::mem::size_of_val(&a2);
    println!("a2 size: {}", std::mem::size_of_val(&a2));

    // 切片
    let a1_slice = &a1[1..3];
    println!("a1_slice: {a1_slice:?}");

    // 数组越界引发 panic
    // println!("a1[5]: {}", a1[5]); // index out of bounds: the length is 5 but the index is 5
}

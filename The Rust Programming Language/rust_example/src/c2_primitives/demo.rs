use core::f32;
use std::{isize, usize};

pub fn run() {
    let i8_max: i8 = std::i8::MAX;
    let i8_min: i8 = std::i8::MIN;
    let i32_max: i32 = std::i32::MAX;
    let i32_min: i32 = std::i32::MIN;
    let isize_max: isize = isize::MAX;
    let isize_min: isize = isize::MIN;
    let u8_max: u8 = std::u8::MAX;
    let u8_min: u8 = std::u8::MIN;
    let usize_max: usize = usize::MAX;
    let usize_min: usize = usize::MIN;
    let f32_max: f32 = f32::MAX;
    let f32_min: f32 = f32::MIN;
    let f64_max: f64 = f64::MAX;
    let f64_min: f64 = f64::MIN;

    let char_a = 'a';

    let bool_t = true;
    let bool_f = false;

    let unit_type = ();

    let arr_i32 = [1, 2, 3];
    let arr_str = ["a", "b", "c"];

    let tup_1 = (1, 2, 3);
    let tup_2 = (1, "a", 3.0);

    println!("i8_max = {i8_max}");
    println!("i8_min = {i8_min}");
    println!("i32_max = {i32_max}");
    println!("i32_min = {i32_min}");
    println!("isize_max = {isize_max}");
    println!("isize_min = {isize_min}");
    println!("u8_max = {u8_max}");
    println!("u8_min = {u8_min}");
    println!("usize_max = {usize_max}");
    println!("usize_min = {usize_min}");
    println!("f32_max = {f32_max}");
    println!("f32_min = {f32_min}");
    println!("f64_max = {f64_max}");
    println!("f64_min = {f64_min}");
    println!("char_a = {char_a}");
    println!("bool_t = {bool_t}");
    println!("bool_f = {bool_f}");
    println!("unit_type = {unit_type:?}");

    println!("arr_i32 = {arr_i32:?}");
    println!("arr_str = {arr_str:?}");

    println!("tup_1 = {tup_1:?}");
    println!("tup_2 = {tup_2:?}");
}

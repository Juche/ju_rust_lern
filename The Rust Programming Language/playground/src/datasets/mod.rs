use std::collections::HashMap;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn mut_fn() {
    let a = 1;
    let mut b = a;
    b += 1;
    println!("a = {}, b = {}", a, b);
}

pub fn vec_fn() {
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    // let v2 = &v[10];
    // println!("v2 = {}", v2);
    match v.get(5) {
        Some(val) => println!("val = {}", val),
        None => println!("None"),
    }

    for i in &v {
        println!("i = {}", i);
    }
}

pub fn hash_fn() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("red"), 10);
    scores.insert(String::from("blue"), 12);

    println!("Scores: {:#?}", scores);
}

pub fn collect_fn() {
    let teams = vec![String::from("red"), String::from("blue")];
    let scores = vec![100, 120];

    let game_tuple = teams.iter().zip(scores.iter());
    println!("Game tuple: {:#?}", game_tuple);

    let game: HashMap<_, _> = game_tuple.collect();
    println!("Game: {:#?}", game);
}

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("T: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

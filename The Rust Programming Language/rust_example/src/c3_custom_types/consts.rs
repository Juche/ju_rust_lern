const THRESHOLD: i32 = 10;
static LANGUAGE: &'static str = "Rust";

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn run() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!(
        "The number {} is {}",
        n,
        if is_big(n) { "big" } else { "small" }
    );
}

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("----- Game start! -----");

    let r_num = rand::thread_rng().gen_range(1..=60);
    // println!("生成的随机数是:{}", r_num);

    loop {
        let mut guess = String::new();

        println!("输入你猜的数字:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Err: {}", err);
                continue;
            }
        };
        println!("你猜的数字是:{}", guess);

        match guess.cmp(&r_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("----- Game over! -----");
}

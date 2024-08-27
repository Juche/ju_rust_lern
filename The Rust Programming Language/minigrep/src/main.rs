use minigrep::read_file;
use minigrep::Input;
use std::env;
use std::process;
// use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);

    // let query = &args[1];
    // let file = &args[2];
    // println!("query = {}", query);
    // println!("file = {}", file);

    let input = Input::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // let content = fs::read_to_string(input.file).expect("文件读取失败!");
    // let content = read_file(input);
    // println!("File content: \n\n{}", content);

    // read_file(input).unwrap_or_else(|e| {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // });
    if let Err(e) = read_file(input) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

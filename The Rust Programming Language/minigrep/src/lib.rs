use std::error::Error;
use std::fs;

pub struct Input {
    pub query: String,
    pub file: String,
}

impl Input {
    // pub fn new(args: &[String]) -> Input {
    pub fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            // panic!("参数不足!");
            return Err("参数不足!");
        }
        let query = args[1].clone();
        let file = args[2].clone();

        Ok(Input { query, file })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn read_file(input: Input) -> Result<(), Box<dyn Error>> {
    // let content = fs::read_to_string(input.file).expect("文件读取失败!");
    let content = fs::read_to_string(input.file)?;
    // println!("File content: \n\n{}", content);

    let result = search(&input.query, &content);
    // println!("result = {:?}", result);

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grep() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}

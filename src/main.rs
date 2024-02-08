use colored::*;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage: {} <{}> <{}>", args[0], "pattern".green(), "file".green());
    }

    let pattern = &args[1].trim();
    let filename = &args[2];
    let contents = std::fs::read_to_string(filename);

    match contents {
        Ok(contents) => color_grep(&contents, pattern),
        Err(err) => println!("{} {}", "Error:".red(), err.to_string())
    }
}

fn color_grep(contents: &str, pattern: &str) {
   for line in contents.lines() {
        if line.contains(pattern) {
            pattern.split_whitespace().for_each(|word| {
                let line = line.replace(word, &format!("{}", word.green()));
                println!("{}", line);
            });
        }
    } 
}

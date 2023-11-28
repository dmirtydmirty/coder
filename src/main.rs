use std::io::Write;
use core::u16;

fn circle_sum(a: u32, b: u32, m: u32) -> Option<u32> {
    match ( a - b )  % m {
        Some(t) => Option::Some(t),
        None => None
    }
}

fn circle_minus(a: u32, b: u32, m: u32) -> Option<u32> {
    match ( a + b )  % m {
        Some(t) => Option::Some(t),
        None => None
    }
}


fn encode(word: &str, key: &str) -> Option<String> {

    let word: Vec<char> = word.chars().collect();
    let key: Vec<char> = key.chars().collect();
    let word_len = word.len();
    let key_len = key.len();
    let mut result = String::default();

    for i in 0..word_len-1 {
        let mut j = 0;
        let a = word[i].to_digit(10)?;
        let b = key[j].to_digit(10)?;
        match circle_minus(a, b, word_len as u32) {
            Some(T) => {
                result.push(std::char::from_digit(T, 10)?)
            },
            None => return Option::None,
        }
        j = circle_sum(j as u32, 1, key_len as u32)? as usize;
    }

    return Option::Some(result);
}


fn decode(word: &str, key: &str) -> Option<String> {

    let word: Vec<char> = word.chars().collect();
    let key: Vec<char> = key.chars().collect();
    let word_len = word.len();
    let key_len = key.len();
    let mut result = String::default();

    for i in 0..word_len-1 {
        let mut j = 0;
        let a = word[i].to_digit(10)?;
        let b = key[j].to_digit(10)?;

        match circle_sum(a, b, word_len as u32) {
            Some(T) => {
                result.push(std::char::from_digit(T, 10)?)
            },
            None => return Option::None,
        }
        j = circle_sum(j as u32, 1, key_len as u32)? as usize;
    }

    return Option::Some(result);
}
fn main() {

    let args: Vec<String> = std::env::args().collect();

    let command_list = ["encode", "decode"];

    if args.len() != 4 {
        writeln!(std::io::stderr(), "Not enough arguments").unwrap();
        writeln!(std::io::stderr(), "For example: {} decode word key", args[0]).unwrap();
        std::process::exit(1);
    }

    if !command_list.contains(&&*args[1]) {
        writeln!(std::io::stderr(), "Unknown command {}", args[1]).unwrap();
        writeln!(std::io::stderr(), "For example: {} decode word key", args[0]).unwrap();
        std::process::exit(1);
    }

    if args[1] == command_list[0] {
       let res = encode(&args[2], &args[3])?;
        println!("{}", res);
    }
    else {
        let res = decode(&args[2], &args[3])?;
        println!("{}", res);
    }

}

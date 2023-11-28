use std::io::Write;
use core::u16;
use log::error;

fn circle_sum(a: u8, b: u8, m: u32) -> Option<u8> {
    let r =( a as u16 + b as u16 )  % m as u16;
    match r {
        t => Option::Some(t as u8),
        _ => None
    }
}

fn circle_minus(a: u8, b: u8, m: u32) -> Option<u8> {
    let r =( a as i16 - b as i16 )  % m as i16;
    match r {
        t => Option::Some(t as u8),
        _ => None
    }
}


fn encode(word: &str, key: &str) -> Option<String> {

    let word: Vec<char> = word.chars().collect();
    let key: Vec<char> = key.chars().collect();
    let word_len = word.len();
    let key_len = key.len();
    let mut result = String::default();
    let mut j = 0;

    for i in 0..word_len {

        let a = word[i];
        let b = key[j];
        let r = circle_sum(a as u8, b as u8, 256 );
        match r {
            Some(T) => {
                result.push(T as char)
            },
            None => return None,
        }
        j = circle_sum(j as u8, 1, key_len as u32)? as usize;
    }

    return Option::Some(result);
}


fn decode(word: &str, key: &str) -> Option<String> {

    let word: Vec<char> = word.chars().collect();
    let key: Vec<char> = key.chars().collect();
    let word_len = word.len();
    let key_len = key.len();
    let mut result = String::default();
    let mut j = 0;

    for i in 0..word_len {

        let a = word[i];
        let b = key[j];
        let r = circle_minus(a as u8, b as u8, 256 );
        match r {
            Some(T) => {
                result.push(T as char)
            },
            None => return None,
        }
        j = circle_sum(j as u8, 1, key_len as u32)? as usize;
    }

    return Option::Some(result);
}
fn main() {

    let mut args: Vec<String> = std::env::args().collect();

    let command_list = ["encode", "decode"];

    if args.len() == 1 {
        args.push("decode".to_string());
        args.push("111".to_string());
        args.push("2".to_string());
    }

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
       match encode(&args[2], &args[3]) {
           Some(T) => println!("{}", T),
           None => {
               writeln!(std::io::stderr(), "Some error").unwrap();
               std::process::exit(1);

           }
       }

    }
    else {
        match decode(&args[2], &args[3]) {
            Some(T) => println!("{}", T),
            None => {
                writeln!(std::io::stderr(), "Some error").unwrap();
                std::process::exit(1);

            }
        }
    }
    let a = 255 as char;
    println!("a: {}", a);
}

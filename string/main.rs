#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    let mut str1 = String::new();
    // single character
    //not double quotes because it is a character not a string
    str1.push('H');
    str1.push_str("ello, ");
    for word in str1.split_whitespace() {
        println!("{}", word);
    }
    let str2 = str1.replace("H", "J");
    println!("{}", str2);

    let str3 = String::from("x r t b h k h");
    // let mut v1: Vec<&str> = str3.split_whitespace().collect();
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for c in v1 {
        print!("{} ", c);
    }
    let str4 = "Rust is fun!";
    let mut str5 = str4.to_string();
    println!("{}", str5);
    let byte_arr = str5.as_bytes();
    let tr6 = &str5[0..4];
    println!("{} len {}", tr6, tr6.len());
    str5.clear();
    let str6 = String::from("and interesting!");
    let str7 = String::from("also exciting!");
    let str8 = str6 + " " + &str7;
    for c in str8.chars() {
        print!("{}", c);
    }
}

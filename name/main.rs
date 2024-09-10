#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    println!("What is your name? ");
    let mut name = String::new();
    let greeting = "Hello, ";
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("{} {}", greeting, name.trim_end());
}

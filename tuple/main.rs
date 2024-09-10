#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    let my_tuple = (18, "Thanh".to_string(), 1_000_000);
    println!("My tuple: {:?}", my_tuple);
    let (age, name, money) = my_tuple;
    println!("Age: {}", age);
    println!("Name: {}", name);
    println!("Money: {}", money);
}

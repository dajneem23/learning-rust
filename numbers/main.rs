#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    println!("Max unsigned 32-bit integer: {}", u32::MAX);
    println!("Max unsigned 64-bit integer: {}", u64::MAX);
    println!("Max unsigned 128-bit integer: {}", u128::MAX);
    println!("Max signed 32-bit integer: {}", i32::MAX);
    println!("Max signed 64-bit integer: {}", i64::MAX);
    println!("Max signed 128-bit integer: {}", i128::MAX);
    println!("Max 32-bit float: {}", f32::MAX);
    println!("Max 64-bit float: {}", f64::MAX);
    // println!("Max 128-bit float: {}", f128::MAX);
    println!("Max char: {}", char::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max isize: {}", isize::MAX);
    println!("Max byte: {}", u8::MAX);
    let num_1: f32 = 1.1111111111111;
    println!("f32: {}", num_1 + 0.1111111111111);
    let num_2: f64 = 1.1111111111111;
    println!("f64: {}", num_2 + 0.1111111111111);
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("num_3 - num_4: {}", num_3 - num_4);
    println!("num_3 + num_4: {}", num_3 + num_4);
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_number);
}

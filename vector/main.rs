#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    vec2.push(6);
    println!("1st element: {}", vec2[0]);
    let second: &i32 = &vec2[1];

    fn get_second(vec: &Vec<i32>) -> Option<&i32> {
        if vec.len() >= 2 {
            Some(&vec[1])
        } else {
            None
        }
    }
    fn check_match(second: Option<&i32>) {
        match second {
            Some(second) => println!("2nd element: {}", second),
            None => println!("No 2nd element."),
        }
    }
    check_match(get_second(&vec2));
    // vec2[1] = 50;
    // println!("2nd element: {}", second);
    for i in &mut vec2 {
        *i += 50;
    }
    for i in &vec2 {
        println!("Element: {}", i);
    }
    println!("Pop: {:?}", vec2.pop());
}

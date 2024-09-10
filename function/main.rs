#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    fn difference(a: i32, b: i32) -> i32 {
        a - b
    }
    fn product(a: i32, b: i32) -> i32 {
        a * b
    }
    fn quotient(a: i32, b: i32) -> i32 {
        a / b
    }
    fn remainder(a: i32, b: i32) -> i32 {
        a % b
    }
    fn sum_list(list: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in list {
            sum += i;
        }
        sum
    }
    let a = 10;
    let b = 5;
    println!("{} + {} = {}", a, b, sum(a, b));
    println!("{} - {} = {}", a, b, difference(a, b));
    println!("{} * {} = {}", a, b, product(a, b));
    println!("{} / {} = {}", a, b, quotient(a, b));
    println!("{} % {} = {}", a, b, remainder(a, b));

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum: {}", sum_list(&num_list));
}

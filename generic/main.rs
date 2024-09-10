#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn get_sum_generic<T: Add<Output = T>>(a: T, b: T) -> T
// where
    // T: Add<Output = T>,
{
    a + b
}

fn main() {
    println!("Sum of 5 and 10: {}", get_sum_generic(5, 10));
    println!("Sum of 5.5 and 10.5: {}", get_sum_generic(5.5, 10.5));
}

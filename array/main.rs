#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    let ar_1 = [1, 2, 3, 4];
    println!("1st element: {}", ar_1[0]);
    println!("2nd element: {}", ar_1[1]);
    println!("Length: {}", ar_1.len());
    let ar_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut loop_index = 0;
    loop {
        if loop_index == ar_2.len() {
            break;
        }
        if ar_2[loop_index] % 2 == 0 {
            println!("{} is even.", ar_2[loop_index]);
        } else {
            println!("{} is odd.", ar_2[loop_index]);
        }
        loop_index += 1;
    }
    loop_index = 0;
    //try with while loop
    while loop_index < ar_2.len() {
        if ar_2[loop_index] % 2 == 0 {
            println!("{} is even.", ar_2[loop_index]);
        } else {
            println!("{} is odd.", ar_2[loop_index]);
        }
        loop_index += 1;
    }
    loop_index = 0;
    //try with for loop
    for element in ar_2.iter() {
        if element % 2 == 0 {
            println!("{} is even.", element);
        } else {
            println!("{} is odd.", element);
        }
    }
    let max_element = ar_2.iter().max();

    match max_element {
        Some(&max) => println!("Max element: {}", max),
        None => println!("No max element."),
    }
}

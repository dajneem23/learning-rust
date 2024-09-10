#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    let age32: i32 = rand::thread_rng().gen_range(0..101);
    match age32 {
        0 => println!("You are a baby."),
        1..=12 => println!("You are a child."),
        13..=19 => println!("You are a teenager."),
        20..=64 => println!("You are an adult."),
        65..=i32::MAX => println!("You are a senior."),
        _ => println!("You are not born yet."),
    }

    let voting_age = 18;
    match age32.cmp(&voting_age) {
        Ordering::Less => println!("You are not old enough to vote."),
        Ordering::Equal => println!("You are old enough to vote."),
        Ordering::Greater => println!("You are old enough to vote."),
    }
}

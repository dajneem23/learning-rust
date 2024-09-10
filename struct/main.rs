#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn main() {
    struct Customer {
        name: String,
        age: u8,
        email: String,
        address: String,
        balance: f64,
    }
    let mut bob = Customer {
        name: String::from("Bob"),
        age: 32,
        email: String::from("bob123@gmail.com"),
        address: String::from("123 Main St"),
        balance: 100.0,
    };
    bob.address = String::from("456 Elm St");
    println!("Bob's address: {}", bob.address);

    struct Rectangle<T, U> {
        width: T,
        height: U,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

}

#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn main() {
    let mut heros: HashMap<_, _> = HashMap::new();
    heros.insert("Superman", 100);
    heros.insert("Batman", 60);
    heros.insert("Aquaman", 80);
    for (key, value) in &heros {
        println!("{}: {}", key, value);
    }
    println!("len: {}", heros.len());
    if heros.contains_key("Superman") {
        println!("Superman is here.");
        let superman = heros.get("Superman");
        match superman {
            Some(&value) => println!("Superman's power: {}", value),
            None => println!("No power."),
        }
    }
}

#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn main() {
    let mut arr_it = [1, 2, 3, 4];
    //borrowing
    //=> no ownership transfer
    //=> can't modify the array
    for val in arr_it.iter() {
        println!("{}", val);
    }
    arr_it
        .into_iter()
        .for_each(|val| println!("into_iter {}", val));

    let mut iter1 = arr_it.iter();

    println!("1st {:?}", iter1.next());
}

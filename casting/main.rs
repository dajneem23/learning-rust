#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};

fn main() {
    //casting
    let int_u8: u8 = 255;
    let int2_u8: u8 = 1;
    let int3_u32: u32 = int_u8 as u32 + int2_u8 as u32;
}

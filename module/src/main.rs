#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
mod restaurant;
use crate::restaurant::order_food;
fn main() {
    //Creates: Modules that prroduce a library or executable
    //Modules: A collection of items: functions, structs, traits, impl blocks, and even other modules or Organize and handle privacy
    //Packages: A Cargo feature that lets you build, test, and share crates
    //Paths: A way of naming an item, such as a struct, function, or module
    order_food();
}

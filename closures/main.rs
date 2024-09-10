#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn main() {
    //similar to functions, but can capture values from the scope in which they're defined
    //let var_name = |param1, param2|->return type { code };
    let add_nums = |num1: i32, num2: i32| -> i32 { num1 + num2 };
    let can_vote = |age: i32| -> bool { age >= 18 };
    println!("Sum: {}", add_nums(5, 5));
    println!("Can Vote: {}", can_vote(18));

    //Closures can capture variables from the scope in which they're defined
    let mut samp1 = 5;
    let print_var = || println!("samp1: {}", samp1);
    print_var();

    //samp1 is borrowed and can't be borrowed again
    let mut change = || samp1 = 10;
    change();
    println!("samp1: {}", samp1);

    //we can pass closures to functions
    let samp2 = 10;
    let multiply = |x| x * samp2;
    println!("3 * 10 = {}", multiply(3));

    fn use_func<T>(a: i32, b: i32, f: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        f(a, b)
    }
    let multiply2 = |x, y| x * y;
    println!("3 * 4 = {}", use_func(3, 4, multiply2));
}

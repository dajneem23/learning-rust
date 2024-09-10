#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
//Stack: LIFO (Last In, First Out) stores data in the order it receives it
//.Data in the stack is stored in a contiguous block of memory with fixed size.
//.The stack is used for static memory allocation.
//.The stack is faster than the heap because it has direct access to the data.
//.The stack is used for function calls and local variables.
//.The stack is limited in size.
//.The stack is thread-safe.
//.The stack is not flexible.
//.The stack is not resizable.

//Heap: Stores data in any order and allows access to any data at any time.
//.Data in the heap is stored in a non-contiguous block of memory with dynamic size.
//.The heap is slower than the stack because it has to look up the data in the memory.
//.The heap is used for dynamic memory allocation.
//.The heap is not limited in size.
//.The heap is not thread-safe.
//.The heap is flexible.
//.The heap is resizable.

//RULES
//1. Each value in Rust has a variable that’s called its owner.
//2. There can only be one owner at a time.
//3. When the owner goes out of scope, the value will be dropped.
//4. The value will be dropped when the owner goes out of scope.
//Ownership Rules
//apply for strings, vectors, files, and other data types.

fn print_string(s: String) {
    println!("string is {}", s);
}
fn print_string_ref(s: &String) {
    println!("string is {}", s);
}
fn print_return_string(s: String) -> String {
    println!("string is {}", s);
    s
}
fn change_string(s: &mut String) {
    s.push_str(" world");
    println!("string is {}", s);
}

fn main() {
    let str1 = String::from("Hello, ");
    let str2 = str1.clone() + "world!";
    println!("{}", str2);
    print_string(str1);
    print_string_ref(&str2);
    let mut str3 = print_return_string(str2);
    println!("{}", str3);
    change_string(&mut str3);
}

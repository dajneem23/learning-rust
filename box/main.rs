#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn main() {
    //Box
    //to store data on the heap rather than the stack
    //useful when you have a large amount of data
    //useful when you have a data with an unknown size at compile time
    //useful when you have a data that you want to own and share across your program
    let b_1: Box<i32> = Box::new(5);
    println!("b_1: {}", b_1);

    struct TreeNode<T> {
        value: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    }
    impl<T> TreeNode<T> {
        pub fn new(value: T) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }
        pub fn add_left(&mut self, value: T) {
            let new_node = TreeNode::new(value);
            self.left = Some(Box::new(new_node));
        }

        pub fn add_right(&mut self, value: T) {
            let new_node = TreeNode::new(value);
            self.right = Some(Box::new(new_node));
        }
    }
    let mut root: TreeNode<i32> = TreeNode::new(5);
    root.add_left(3);
    root.add_right(7);
    println!("Root value: {}", root.value);
    match root.left {
        Some(ref left) => println!("Left value: {}", left.value),
        None => println!("No left value."),
    }
    match root.right {
        Some(ref right) => println!("Right value: {}", right.value),
        None => println!("No right value."),
    }
}

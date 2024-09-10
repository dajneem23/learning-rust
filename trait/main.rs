#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
fn main() {
    trait Shape<T, U> {
        fn new(width: T, height: U) -> Self;
        fn area(&self) -> T;
        fn perimeter(&self) -> U;
    }
    struct Rectangle<T, U> {
        width: T,
        height: U,
    }
    struct Circle<T> {
        radius: T,
    }
    impl<T, U> Shape<T, U> for Rectangle<T, U>
    where
        T: Mul<Output = T> + Copy,
        U: Add<Output = U> + Copy,
    {
        fn new(width: T, height: U) -> Self {
            Rectangle { width, height }
        }
        fn area(&self) -> T {
            self.width * self.width
        }
        fn perimeter(&self) -> U {
            self.height + self.height
        }
    }
    impl Shape<f32, f32> for Circle<f32> {
        fn new(radius: f32, _height: f32) -> Self {
            Circle { radius }
        }
        fn area(&self) -> f32 {
            std::f32::consts::PI * self.radius * self.radius
        }
        fn perimeter(&self) -> f32 {
            2.0 * std::f32::consts::PI * self.radius
        }
    }
    let rec = Rectangle::new(5, 10);
    println!("Rectangle area: {}", rec.area());
    println!("Rectangle perimeter: {}", rec.perimeter());
    let cir = Circle::new(5.0, 0.0);
    println!("Circle area: {}", cir.area());
    println!("Circle perimeter: {}", cir.perimeter());
}

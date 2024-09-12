#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    //Threads: Enables multiple paths of execution
    //Concurrency: The ability to run multiple programs at the same time
    //Parallelism: Running multiple programs at the same time
    //Message passing: Communication between threads
    //Deadlock: Two or more threads are waiting on each other to release a resource

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    let v = vec![1, 2, 3];
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    pub struct Bank {
        balance: f32,
    }
    fn withdraw(bank: &mut Bank, amount: f32) {
        bank.balance -= amount;
    }
    let mut bank = Bank { balance: 100.0 };

    withdraw(&mut bank, 5.00);

    println!("Bank balance: {}", bank.balance);

    fn customer(bank: &mut Bank) {
        withdraw(bank, 5.00);
    }

    //move ownership of bank to the thread
    // || is a closure
    thread::spawn(move || {
        customer(&mut bank);
    })
    .join()
    .unwrap();
}

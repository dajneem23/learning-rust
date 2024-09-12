#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    pub struct Bank {
        balance: i32,
    }
    fn withdraw(bank: &Arc<Mutex<Bank>>, amount: i32) {
        let mut bank: std::sync::MutexGuard<'_, Bank> = bank.lock().unwrap();
        if bank.balance >= amount {
            bank.balance -= amount;
            println!("Withdrew: {}", amount);
        } else {
            println!("Insufficient funds.");
        }
    }
    fn customer(bank: Arc<Mutex<Bank>>, amount: i32) {
        withdraw(&bank, amount);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 100 }));
    let handles = (0..10)
        .map(|i| {
            //2 different ways to clone the Arc<Mutex<Bank>> to pass to the thread
            // let bank = Arc::clone(&bank);
            let bank = bank.clone();
            thread::spawn(move || {
                customer(bank, i * 10);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final balance: {}", bank.lock().unwrap().balance);
}

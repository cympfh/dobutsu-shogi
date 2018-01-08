#![allow(unused_imports)]
use std::io::{ self, Write };
use std::str::FromStr;
use std::cmp::{ min, max };
use std::collections::{ BinaryHeap, VecDeque };

extern crate rand;

enum Piece {
    Chick, Chicken,
    Lion, Elephant, Giraffe
}

enum Owner {
    Next, Prev
}

struct Field (Vec<Vec<(Piece, Owner)>>);

#[allow(unused_macros)]
macro_rules! trace {
    ($var:expr) => {
        let _ = writeln!(&mut std::io::stderr(), ">>> {} = {:?}", stringify!($var), $var);
    };
    ($($args:expr),*) => { trace!(($($args),*)) }
}

#[allow(unused_macros)]
macro_rules! put {
    ($var:expr) => {
        let _ = writeln!(&mut std::io::stdout(), "{}", $var);
    };
    ($var:expr, $($args:expr),*) => {
        let _ = write!(&mut std::io::stdout(), "{} ", $var);
        put!($($args),*);
    };
}

const M: usize = 1_000_000_007;

fn main() {
    let mut sc = Scanner::new();
}

#[allow(dead_code)]
struct Scanner { stdin: io::Stdin, buffer: VecDeque<String>, }
#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner { Scanner { stdin: io::stdin(), buffer: VecDeque::new() } }
    fn reserve(&mut self) {
        while self.buffer.len() == 0 {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        self.reserve();
        match self.buffer.pop_front().unwrap().parse::<T>() {
            Ok(a) => a,
            Err(_) => panic!("parse err")
        }
    }
}

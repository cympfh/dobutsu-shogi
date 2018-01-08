#![allow(unused_imports)]
use std::env;
use std::io::{ self, Write };
use std::str::FromStr;
use std::cmp::{ min, max };
use std::collections::{ BinaryHeap, VecDeque };

extern crate rand;

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

/**
 * Data Structures
 */

#[derive(Clone, Debug, PartialEq)]
enum Piece {
    Chick, Chicken,
    Lion, Elephant, Giraffe
}
use Piece::{Chick, Chicken, Lion, Elephant, Giraffe};

#[derive(Clone, Debug)]
enum Player {
    Next, Prev
}
use Player::{Next, Prev};

fn char2piece(c: char) -> Option<Piece> {
    if c == 'L' {
        Some(Lion)
    } else if c == 'C' {
        Some(Chick)
    } else if c == 'D' {
        Some(Chicken)
    } else if c == 'E' {
        Some(Elephant)
    } else if c == 'G' {
        Some(Giraffe)
    } else {
        None
    }
}

fn piece2char(p: Piece) -> char {
    match p {
        Chick => 'C',
        Chicken => 'D',
        Elephant => 'E',
        Giraffe => 'G',
        Lion => 'L',
    }
}

fn char2piece_with_owner(c: char, d: char) -> Option<(Player, Piece)> {
    if c == 'P' {
        Some((Prev, char2piece(d).unwrap()))
    } else if c == 'N' {
        Some((Next, char2piece(d).unwrap()))
    } else {
        None
    }
}

struct State {
    field: Vec<Vec<Option<(Player, Piece)>>>,
    keep_next: Vec<Piece>,
    keep_prev: Vec<Piece>
}

impl State {

    // fn new() -> State {
    //     let mut f = vec![vec![None; 3]; 4];
    //     f[0][0] = Some((Prev, Giraffe));
    //     f[0][1] = Some((Prev, Lion));
    //     f[0][2] = Some((Prev, Elephant));
    //     f[1][1] = Some((Prev, Chick));
    //     f[2][1] = Some((Next, Chick));
    //     f[3][0] = Some((Next, Elephant));
    //     f[3][1] = Some((Next, Lion));
    //     f[3][2] = Some((Next, Giraffe));
    //     State { field: f, keep_next: vec![], keep_prev: vec![] }
    // }

    fn read() -> State {
        let mut sc = Scanner::new();

        let mut f = vec![vec![None; 3]; 4];
        for i in 0..4 {
            let cs: Vec<char> = sc.cin::<String>().chars().collect();
            for j in 0..3 {
                let p = char2piece_with_owner(cs[2 * j], cs[2 * j + 1]);
                f[i][j] = p;
            }
        }

        let mut kn = vec![];
        {
            let cs: Vec<char> = sc.cin::<String>().chars().collect();
            for c in cs {
                if let Some(p) = char2piece(c) {
                    kn.push(p);
                }
            }
        }

        let mut kp = vec![];
        {
            let cs: Vec<char> = sc.cin::<String>().chars().collect();
            for c in cs {
                if let Some(p) = char2piece(c) {
                    kp.push(p);
                }
            }
        }

        State { field: f, keep_next: kn, keep_prev: kp }
    }

    fn print(&self) {

        // field
        for i in 0..4 {
            for j in 0..3 {
                if let Some((ref owner, ref p)) = self.field[i][j] {
                    match owner {
                        &Next => print!("N"),
                        &Prev => print!("P")
                    }
                    print!("{}", piece2char(p.clone()));
                } else {
                    print!("..")
                }
            }
            println!("");
        }

        // keeps
        if self.keep_next.len() > 0 {
            for p in self.keep_next.iter() {
                print!("{}", piece2char(p.clone()));
            }
            println!("");
        } else {
            println!(".");
        }

        if self.keep_prev.len() > 0 {
            for p in self.keep_prev.iter() {
                print!("{}", piece2char(p.clone()));
            }
            println!("");
        } else {
            println!(".");
        }
    }

    fn check(&self) -> Option<Player> {
        for p in self.keep_next.iter() {
            if p == &Lion {
                return Some(Next)
            }
        }
        for p in self.keep_prev.iter() {
            if p == &Lion {
                return Some(Prev)
            }
        }
        None
    }

}

fn usage() {
    println!(r#"dshogi

Usage:
    dshogi [COMMAND]

COMMAND:

    solve
    play
    check
"#);

}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {

        usage();

    } else if args[1] == "check" {

        let s = State::read();
        if let Some(win_player) = s.check() {
            match win_player {
                Next => println!("N"),
                Prev => println!("P"),
            }
        } else {
            println!("going")
        }

    } else {

        println!("unknown command: {}", args[1]);
        usage();

    }
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

#![feature(associated_types, slicing_syntax)]

extern crate test;
extern crate time;

use std::collections::HashMap;
use std::io::stdio;
use std::os;

struct Primes {
    map: HashMap<u32, u32>,
    n: u32,
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        loop {
            self.n += 1;

            let q = self.n;

            match self.map.remove(&q) {
                None => {
                    self.map.insert(q * q, q);

                    return Some(q);
                },
                Some(p) => {
                    let mut x = p + q;

                    while self.map.contains_key(&x) {
                        x += p;
                    }

                    self.map.insert(x, p);
                },
            }
        }
    }
}

fn primes(capacity: uint) -> Primes {
    Primes {
        map: HashMap::with_capacity(capacity),
        n: 1,
    }
}

fn solution() -> u32 {
    let target = 10_000;

    primes(target).nth(target).unwrap()
}

fn main() {
    match os::args()[] {
        [_, ref flag] if flag[] == "-a" => return println!("{}", solution()),
        _ => {},
    }

    for line in stdio::stdin().lock().lines() {
        let iters: u64 = line.unwrap()[].trim().parse().unwrap();

        let start = time::precise_time_ns();
        for _ in range(0, iters) {
            test::black_box(solution());
        }
        let end = time::precise_time_ns();

        println!("{}", end - start);
    }
}

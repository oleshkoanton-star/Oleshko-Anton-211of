// https://www.hackerrank.com/challenges/staircase/problem
use std::io;

fn staircase(n: i32) {
    for i in 1..=n {
        println!("{}{}", " ".repeat((n - i) as usize), "#".repeat(i as usize));
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: i32 = input.trim().parse().unwrap();
    staircase(n);
}

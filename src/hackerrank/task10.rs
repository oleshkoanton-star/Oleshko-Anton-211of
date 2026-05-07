// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;
use std::io;

fn sock_merchant(arr: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for sock in arr {
        *map.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for count in map.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let mut input = String::new();

    // n
    io::stdin().read_line(&mut input).unwrap();

    input.clear();

    // socks
    io::stdin().read_line(&mut input).unwrap();

    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = sock_merchant(arr);

    println!("{}", result);
}

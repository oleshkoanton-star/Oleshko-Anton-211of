// https://www.hackerrank.com/challenges/between-two-sets/problem
use std::io;

// gcd
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// lcm
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    // lcm of array a
    let mut l = a[0];
    for &num in &a[1..] {
        l = lcm(l, num);
    }

    // gcd of array b
    let mut g = b[0];
    for &num in &b[1..] {
        g = gcd(g, num);
    }

    // count multiples of l that divide g
    let mut count = 0;
    let mut multiple = l;

    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

fn main() {
    let mut input = String::new();

    // n m
    io::stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();

    // array a
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();

    // array b
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = get_total_x(a, b);
    println!("{}", result);
}

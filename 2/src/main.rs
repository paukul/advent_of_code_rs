#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::fs;
use test::Bencher;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not read input");
    let ids = input.split("\n")
        .collect::<Vec<&str>>();
    let result = sum(ids);
    println!("Result: {:?}", result);
}

fn calc(ids: &str) -> (i32, i32) {
    let mut counts = HashMap::new();

    for c in ids.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    counts.values().fold((0, 0), |mut acc, val| {
        match val {
            2 => acc.0 = 1,
            3 => acc.1 = 1,
            _ => {}
        }
        acc
    })
}

fn sum(ids: Vec<&str>) -> i32 {
    let sums = ids
        .iter()
        .map(|id| calc(id))
        .fold((0, 0), |acc, counts| (acc.0 + counts.0, acc.1 + counts.1));
    sums.0 * sums.1
}

#[test]
fn test_counting_with_zero() {
    assert_eq!(calc("abcdef"), (0, 0));
    assert_eq!(calc("bababc"), (1, 1));
    assert_eq!(calc("abbcde"), (1, 0));
    assert_eq!(calc("abcccd"), (0, 1));
    assert_eq!(calc("aabcdd"), (1, 0));
    assert_eq!(calc("abcdee"), (1, 0));
    assert_eq!(calc("ababab"), (0, 1));
}

#[test]
fn test_suming_up() {
    let seq = vec![
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    assert_eq!(sum(seq), 12);
}

#[bench]
fn bench_counting(b: &mut Bencher) {
    b.iter(|| calc("abcdef"))
}

#[bench]
fn bench_counting_long(b: &mut Bencher) {
    b.iter(|| calc("abcdefasdfasdflkasdfjlkasdjflkansblksdnbsalkdbnldsakgjlkjasbalskdbhsdalkbasdlkbhbasdlkbdlks"))
}

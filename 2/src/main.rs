#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::fs;
#[allow(unused_imports)]
use test::Bencher;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input");
    let ids = input.lines().collect::<Vec<&str>>();
    let result = sum(&ids);
    println!("2.a: {:?}", result);
    println!("2.b: {:?}", find_partial(ids));
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

fn sum(ids: &[&str]) -> i32 {
    let sums = ids
        .iter()
        .map(|id| calc(id))
        .fold((0, 0), |acc, counts| (acc.0 + counts.0, acc.1 + counts.1));
    sums.0 * sums.1
}

#[allow(dead_code)]
fn similars(ids: Vec<&str>) -> (&str, &str) {
    ids.iter()
        .enumerate()
        .find_map(|(index, &id)| {
            let head = &ids[0..index];
            let tail = &ids[(index + 1)..];

            let found = head.iter().chain(tail.iter()).find(|&comparison_id| {
                let diff =
                    comparison_id
                        .chars()
                        .zip(id.chars())
                        .fold(0, |mut diff, (left, right)| {
                            if left != right {
                                diff += 1;
                            }
                            diff
                        });

                diff == 1
            });
            found.map(|&f| (id, f))
        })
        .unwrap()
}

fn same_partial(left: &str, right: &str) -> String {
    left.chars()
        .zip(right.chars())
        .filter(|&(l, r)| l == r)
        .map(|(c, _)| c)
        .collect()
}

fn find_partial(ids: Vec<&str>) -> String {
    let (first, second) = similars(ids);
    same_partial(first, second)
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
    assert_eq!(sum(&seq), 12);
}

#[test]
fn test_finding_similars() {
    let seq = vec![
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    assert_eq!(similars(seq), ("fghij", "fguij"));
}

#[test]
fn test_same_partial() {
    assert_eq!(same_partial("fghij", "fguij"), "fgij".to_owned())
}

#[test]
fn test_find_partial() {
    let seq = vec![
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    assert_eq!(find_partial(seq), "fgij".to_owned())
}

#[bench]
fn bench_counting(b: &mut Bencher) {
    b.iter(|| calc("abcdef"))
}

#[bench]
fn bench_counting_long(b: &mut Bencher) {
    b.iter(|| calc("abcdefasdfasdflkasdfjlkasdjflkansblksdnbsalkdbnldsakgjlkjasbalskdbhsdalkbasdlkbhbasdlkbdlks"))
}

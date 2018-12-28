#![allow(dead_code)]
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::error;
use std::fs;

type Result<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug, PartialEq)]
struct Claim {
    id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn expand(&self) -> Vec<String> {
        let x_2 = self.x + self.width;
        let y_2 = self.y + self.height;
        let mut expands: Vec<String> = Vec::new();
        for x in self.x..x_2 {
            for y in self.y..y_2 {
                expands.push(format!("{},{}", x, y));
            }
        }
        expands
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    println!("Cords within two or more: {}", two_or_more(&lines));
    println!("ID of not overlapping claim: {}", none_overlapping_claim(&lines).id);
}

fn two_or_more(claims: &[&str]) -> i32 {
    set_claims(claims).iter().fold(0, |mut sum, (_, &v)| {
        if v > 1 {
            sum += 1
        }
        sum
    })
}

fn none_overlapping_claim(claims: &[&str]) -> Claim {
    let fabric = set_claims(claims);
    claims
        .iter()
        .map(|&claim_str| parse_claim(claim_str))
        .flatten()
        .find(|claim| {
            claim.expand().iter().all(|coord| {
                if let Some(&claim_count) = fabric.get(coord) {
                    return claim_count < 2;
                }
                false
            })
        })
        .unwrap()
}

fn set_claims(claims: &[&str]) -> HashMap<String, i32> {
    let mut fabric = HashMap::new();
    claims
        .iter()
        .map(|&claim_str| parse_claim(claim_str))
        .filter_map(Option::Some)
        .flatten()
        .map(|claim| claim.expand())
        .flatten()
        .for_each(|claim_cord| {
            *fabric.entry(claim_cord).or_insert(0) += 1;
        });

    fabric
}

fn parse_claim(claim: &str) -> Option<Claim> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<width>\d+)x(?P<height>\d+)")
                .unwrap();
    }
    RE.captures(claim).map(|caps| {
        // this should never panic because the regexp matched
        Claim {
            id: caps["id"].to_owned(),
            x: caps["x"].parse::<i32>().unwrap(),
            y: caps["y"].parse::<i32>().unwrap(),
            width: caps["width"].parse::<i32>().unwrap(),
            height: caps["height"].parse::<i32>().unwrap(),
        }
    })
}

#[test]
fn test_parse_claim() {
    let expected = Claim {
        id: "1".to_owned(),
        x: 1,
        y: 3,
        width: 4,
        height: 4,
    };
    let result = parse_claim("#1 @ 1,3: 4x4");
    assert_eq!(result.unwrap(), expected);
}

#[allow(unused_variables)]
#[test]
fn test_expand_claim() {
    let claim = Claim {
        id: "1".to_owned(),
        x: 1,
        y: 1,
        width: 2,
        height: 2,
    };
    let expected = vec![
        "1,1".to_owned(),
        "1,2".to_owned(),
        "2,1".to_owned(),
        "2,2".to_owned(),
    ];
    assert_eq!(claim.expand(), expected);
}

#[test]
fn test_example() {
    let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];

    let result = set_claims(&claims);
    let mut expected = HashMap::new();
    expected.insert("4,1".to_owned(), 1);
    expected.insert("5,1".to_owned(), 1);
    expected.insert("6,1".to_owned(), 1);
    expected.insert("7,1".to_owned(), 1);
    expected.insert("4,2".to_owned(), 1);
    expected.insert("5,2".to_owned(), 1);
    expected.insert("6,2".to_owned(), 1);
    expected.insert("7,2".to_owned(), 1);
    expected.insert("1,3".to_owned(), 1);
    expected.insert("2,3".to_owned(), 1);
    expected.insert("3,3".to_owned(), 2);
    expected.insert("4,3".to_owned(), 2);
    expected.insert("5,3".to_owned(), 1);
    expected.insert("6,3".to_owned(), 1);
    expected.insert("1,4".to_owned(), 1);
    expected.insert("2,4".to_owned(), 1);
    expected.insert("3,4".to_owned(), 2);
    expected.insert("4,4".to_owned(), 2);
    expected.insert("5,4".to_owned(), 1);
    expected.insert("6,4".to_owned(), 1);
    expected.insert("2,5".to_owned(), 1);
    expected.insert("3,5".to_owned(), 1);
    expected.insert("4,5".to_owned(), 1);
    expected.insert("5,5".to_owned(), 1);
    expected.insert("6,5".to_owned(), 1);
    expected.insert("1,6".to_owned(), 1);
    expected.insert("2,6".to_owned(), 1);
    expected.insert("3,6".to_owned(), 1);
    expected.insert("4,6".to_owned(), 1);
    expected.insert("5,6".to_owned(), 1);
    expected.insert("6,6".to_owned(), 1);

    assert_eq!(result, expected);
}

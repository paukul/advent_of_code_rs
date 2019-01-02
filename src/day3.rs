use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    expands: Vec<String>,
}

impl Claim {
    fn new(claim_str: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<width>\d+)x(?P<height>\d+)"
            )
            .unwrap();
        }
        let caps = RE.captures(claim_str).unwrap();
        let x = caps["x"].parse::<i32>().unwrap();
        let y = caps["y"].parse::<i32>().unwrap();
        let width = caps["width"].parse::<i32>().unwrap();
        let height = caps["height"].parse::<i32>().unwrap();
        let x_2 = x + width;
        let y_2 = y + height;

        let slots: usize = (width + height + 2) as usize;
        let mut expands: Vec<String> = Vec::with_capacity(slots);
        for x in x..x_2 {
            for y in y..y_2 {
                expands.push(format!("{},{}", x, y));
            }
        }

        Claim {
            id: caps["id"].to_owned(),
            x: x,
            y: y,
            width: width,
            height: height,
            expands: expands,
        }
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Claim> {
    input.lines().map(|line| Claim::new(line)).collect()
}

#[aoc(day3, part1)]
pub fn two_or_more(claims: &[Claim]) -> i32 {
    set_claims(claims).iter().fold(0, |mut sum, (_, &v)| {
        if v > 1 {
            sum += 1
        }
        sum
    })
}

#[aoc(day3, part2)]
pub fn none_overlapping_claim(claims: &[Claim]) -> String {
    let fabric = set_claims(claims);
    claims
        .iter()
        .find(|&claim| {
            claim.expands.iter().all(|coord| {
                if let Some(&claim_count) = fabric.get(coord.as_str()) {
                    return claim_count < 2;
                }
                false
            })
        })
        .unwrap()
        .id
        .clone()
}

fn set_claims(claims: &[Claim]) -> HashMap<&str, i32> {
    let mut fabric = HashMap::new();
    claims
        .iter()
        .map(|claim| &claim.expands)
        .flatten()
        .for_each(|claim_cord| {
            *fabric.entry(claim_cord.as_str()).or_insert(0) += 1;
        });

    fabric
}

#[test]
fn test_parse_claim() {
    let expected = Claim {
        id: "1".to_owned(),
        x: 1,
        y: 3,
        width: 1,
        height: 1,
        expands: vec!["1,3".to_owned()],
    };
    let result = Claim::new("#1 @ 1,3: 1x1");
    assert_eq!(result, expected);
}

#[allow(unused_variables)]
#[test]
fn test_expand_claim() {
    let claim = Claim::new("#1 @ 1,1: 2x2");
    let expected = vec![
        "1,1".to_owned(),
        "1,2".to_owned(),
        "2,1".to_owned(),
        "2,2".to_owned(),
    ];
    assert_eq!(claim.expands, expected);
}

#[test]
fn test_example() {
    let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
        .iter()
        .map(|&claim_str| Claim::new(claim_str))
        .collect::<Vec<Claim>>();

    let result = set_claims(&claims);
    let mut expected = HashMap::new();
    expected.insert("4,1", 1);
    expected.insert("5,1", 1);
    expected.insert("6,1", 1);
    expected.insert("4,2", 1);
    expected.insert("5,2", 1);
    expected.insert("6,2", 1);
    expected.insert("1,3", 1);
    expected.insert("2,3", 1);
    expected.insert("3,3", 2);
    expected.insert("4,3", 2);
    expected.insert("5,3", 1);
    expected.insert("6,3", 1);
    expected.insert("1,4", 1);
    expected.insert("2,4", 1);
    expected.insert("3,1", 1);
    expected.insert("3,2", 1);
    expected.insert("3,4", 2);
    expected.insert("4,4", 2);
    expected.insert("5,4", 1);
    expected.insert("6,4", 1);
    expected.insert("1,5", 1);
    expected.insert("2,5", 1);
    expected.insert("3,5", 1);
    expected.insert("4,5", 1);
    expected.insert("5,5", 1);
    expected.insert("6,5", 1);
    expected.insert("1,6", 1);
    expected.insert("2,6", 1);
    expected.insert("3,6", 1);
    expected.insert("4,6", 1);
    expected.insert("5,6", 1);
    expected.insert("6,6", 1);
    for (k, v) in &expected {
        if let Some(v2) = result.get(k) {
            if v != v2 {
                println!("Key {} is {} should be {}", k, v2, v)
            }
        } else {
            println!("Key {} missing", k);
        }
    }

    for (k, _) in &result {
        if expected.get(k).is_none() {
            println!("Extranous key {}", k);
        }
    }

    assert_eq!(result, expected);
}

use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn solve_part_a(input: &str) -> i32 {
    let sums = input
        .lines()
        .map(|id| calc(id))
        .fold((0, 0), |acc, counts| (acc.0 + counts.0, acc.1 + counts.1));
    sums.0 * sums.1
}

#[aoc(day2, part2)]
pub fn solve_part_b(input: &str) -> String {
    let (first, second) = similars(input);
    same_partial(first, second)
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

fn similars(input: &str) -> (&str, &str) {
    let ids: Vec<&str> = input.lines().collect();
    ids
      .iter()
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
    let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    assert_eq!(solve_part_a(input), 12);
}

#[test]
fn test_finding_similars() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
    assert_eq!(similars(input), ("fghij", "fguij"));
}

#[test]
fn test_same_partial() {
    assert_eq!(same_partial("fghij", "fguij"), "fgij".to_owned())
}

#[test]
fn test_find_partial() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
    assert_eq!(solve_part_b(input), "fgij".to_owned())
}

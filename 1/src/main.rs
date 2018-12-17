use std::fs;

fn main() {
    let sequence = fs::read_to_string("input.txt").expect("could not read file");
    println!("{:?}", calc(&sequence));
}

fn calc(sequence: &str) -> i32 {
    sequence
        .split('\n')
        .filter(|frequency| !frequency.is_empty())
        .map(|frequency| frequency.trim().parse::<i32>().unwrap())
        .sum()
}

#[test]
fn test_first_example() {
    assert_eq!(calc("+1\n+1\n+1"), 3);
    assert_eq!(calc("+1\n+1\n-2"), 0);
    assert_eq!(calc("-1\n-2\n-3"), -6);
}

use std::fs;
use std::collections::HashSet;

fn main() {
    let sequence = fs::read_to_string("input.txt").expect("could not read file");
    println!("1.a: {:?}", calc(&sequence));
    println!("1.b: {:?}", calc_b(&sequence));
}

fn calc(sequence: &str) -> i32 {
    get_vec(sequence)
        .iter()
        .sum()
}

fn calc_b(sequence: &str) -> i32 {
    let mut frequencies = get_vec(sequence);
    let mut sums = HashSet::new();
    let mut current = 0;
    sums.insert(current);

    loop {
        let next = frequencies.remove(0);
        frequencies.push(next);
        current += next;
        if sums.contains(&current) {
            break;
        }
        sums.insert(current);
    }

    current
}

fn get_vec(sequence: &str) -> Vec<i32> {
    sequence.split('\n')
        .map(|frequency| frequency.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[test]
fn test_first_example() {
    assert_eq!(calc("+1\n+1\n+1"), 3);
    assert_eq!(calc("+1\n+1\n-2"), 0);
    assert_eq!(calc("-1\n-2\n-3"), -6);
}

#[test]
fn test_second_example() {
    use std::collections::HashMap;

    let mut inputs = HashMap::new();
    inputs.insert(0, "+1\n-1");
    inputs.insert(10, "+3\n+3\n+4\n-2\n-4");
    inputs.insert(5, "-6\n+3\n+8\n+5\n-6");
    inputs.insert(14, "+7\n+7\n-2\n-7\n-4");
    for (&result, input) in inputs.iter() {
        assert_eq!(calc_b(input), result);
    }
}

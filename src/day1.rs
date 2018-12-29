use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn get_vec(lines: &str) -> Vec<i32> {
    lines
        .lines()
        .map(|frequency| frequency.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[aoc(day1, part1)]
pub fn solve_part_1(input: &[i32]) -> i32 {
  input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &[i32]) -> i32 {
    let mut sums = HashSet::new();
    let mut current = 0;
    sums.insert(current);

    for next in input.iter().cycle() {
        current += next;
        if sums.contains(&current) {
            break;
        }
        sums.insert(current);
    }

    current
}

#[test]
fn test_first_example() {
    assert_eq!(solve_part_1(&get_vec("+1\n+1\n+1")), 3);
    assert_eq!(solve_part_1(&get_vec("+1\n+1\n-2")), 0);
    assert_eq!(solve_part_1(&get_vec("-1\n-2\n-3")), -6);
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
        assert_eq!(solve_part_2(&get_vec(input)), result);
    }
}

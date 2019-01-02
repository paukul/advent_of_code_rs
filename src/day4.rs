extern crate chrono;
use chrono::NaiveDateTime;

static FORMAT: &str = "%Y-%m-%d %H:%M";

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(NaiveDateTime, &str)> {
    let mut lines_with_time: Vec<(NaiveDateTime, &str)> = input
        .lines()
        .map(|line| NaiveDateTime::parse_from_str(&line[1..17], FORMAT).map(|time| (time, line)))
        .flatten()
        .collect();

    lines_with_time.sort_by(|(time_a, _), (time_b, _)| time_a.cmp(time_b));

    lines_with_time
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_generator() {
        let input = vec![
            "[1518-09-29 00:35] falls asleep",
            "[1518-09-25 00:28] falls asleep",
            "[1518-10-14 00:00] Guard #2927 begins shift",
            "[1518-07-20 00:53] wakes up",
        ]
        .join("\n");
        let raw_result = generator(&input);
        let line_result: Vec<&str> = raw_result.iter().map(|&(_, line)| line).collect();
        let expected = vec![
            "[1518-07-20 00:53] wakes up",
            "[1518-09-25 00:28] falls asleep",
            "[1518-09-29 00:35] falls asleep",
            "[1518-10-14 00:00] Guard #2927 begins shift",
        ];
        assert_eq!(expected, line_result);
        let expected_time = NaiveDate::from_ymd(1518, 7, 20).and_hms(0, 53, 0);
        assert_eq!(raw_result[0].0, expected_time);
    }
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let reports = input
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    //solution a
    let result_a = reports
        .iter()
        .filter(|levels| {
            let diffs = levels
                .iter()
                .zip(levels.iter().skip(1))
                .map(|(first, second)| second - first)
                .collect::<Vec<_>>();
            (diffs.iter().all(|v| *v > 0) || diffs.iter().all(|v| *v < 0))
                && diffs.iter().all(|v| v.abs() <= 3)
        })
        .count() as i64;
    //solution b
    let result_b = reports.iter().filter(|report| test_report(report)).count() as i64;

    (result_a, result_b)
}

fn first_fault(report: &[i64]) -> Option<usize> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(first, second)| ((second - first).signum(), (second - first).abs()))
        .scan((0, false), |(sign, change), (diff_sign, diff_magnitude)| {
            match *sign {
                -1 => {
                    if diff_sign > 0 {
                        *change = true
                    }
                }
                0 => {}
                1 => {
                    if diff_sign < 0 {
                        *change = true
                    }
                }
                _ => panic!(),
            }
            *sign = diff_sign;
            Some(*change || !(1..=3).contains(&diff_magnitude))
        })
        .position(|b| b)
}

fn test_report(report: &[i64]) -> bool {
    first_fault(report).map_or(true, |index| {
        (index.saturating_sub(1)..=(index + 1)).any(|index| {
            let mut dropped = report.to_vec();
            dropped.remove(index);
            first_fault(&dropped).is_none()
        })
    })
}

#[test]
fn test_solution() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .lines()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(solution(input), (2, 4));
}

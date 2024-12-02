use std::collections::HashMap;

use itertools::Itertools;

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let (left, right): (Vec<i64>, Vec<i64>) = input
        .iter()
        .map(|s| {
            let mut a = s.split_whitespace();
            (
                a.next().unwrap().parse::<i64>().unwrap(),
                a.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .unzip();

    //solution A
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();
    let result_a = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    //solution B
    let r = right_sorted.iter().dedup_with_count().map(|(count,value)|(*value,count as i64)).collect::<HashMap<_,_>>();
    let result_b= left_sorted.iter().dedup_with_count().map(|(count,value)| *value*(count as i64)*r.get(value).unwrap_or(&0)).sum();
    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3"
        .lines()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(solution(input), (11, 31));
}

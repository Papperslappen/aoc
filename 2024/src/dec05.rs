use std::{cmp::Ordering, collections::HashMap};

use nom::{
    self,
    bytes::complete::{tag, take_while_m_n},
    character::complete::newline,
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, Tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct PageOrderRules {
    order: HashMap<u32, Vec<u32>>,
}

impl PageOrderRules {
    fn from_pairs(pairs: Vec<(u32, u32)>) -> PageOrderRules {
        let mut order = HashMap::new();
        for (first, second) in pairs {
            order
                .entry(first)
                .and_modify(|v: &mut Vec<u32>| v.push(second))
                .or_insert(vec![second]);
        }
        PageOrderRules { order }
    }

    fn validate_print_order(&self, print_order: &[u32]) -> bool {
        (0..print_order.len()).all(|i| {
            let mid = print_order[i];
            let before = &print_order[0..i];
            let after = &print_order[i + 1..];
            before
                .iter()
                .all(|first| self.order.get(first).map_or(true, |f| f.contains(&mid)))
                && !after
                    .iter()
                    .any(|last| self.order.get(last).map_or(false, |f| f.contains(&mid)))
        })
    }
    fn ordering(&self, a: u32, b: u32) -> Option<Ordering> {
        match (self.order.get(&a), self.order.get(&b)) {
            (None, None) => None,
            (None, Some(v)) => {
                if v.contains(&a) {
                    Some(Ordering::Greater)
                } else {
                    None
                }
            }
            (Some(v), None) => {
                if v.contains(&b) {
                    Some(Ordering::Less)
                } else {
                    None
                }
            }
            (Some(v1), Some(v2)) => {
                if v1.contains(&b) {
                    Some(Ordering::Less)
                } else if v2.contains(&a) {
                    Some(Ordering::Greater)
                } else {
                    None
                }
            }
        }
    }
}

fn two_digit_int(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 2, |c: char| c.is_ascii_digit()),
        str::parse,
    )
    .parse(input)
}

fn pair_of_ints(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(two_digit_int, tag("|"), two_digit_int).parse(input)
}

fn page_order_rules_parser(input: &str) -> IResult<&str, PageOrderRules> {
    let (rest, order) = separated_list1(newline, pair_of_ints).parse(input)?;
    Ok((rest, PageOrderRules::from_pairs(order)))
}

fn print_order(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), two_digit_int).parse(input)
}

fn print_orders_parser(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(newline, print_order).parse(input)
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let input = input.join("\n");
    let (_, (page_order_rules, print_orders)) = (
        page_order_rules_parser,
        preceded(many1(newline), print_orders_parser),
    )
        .parse(&input)
        .unwrap();
    let result_a = print_orders
        .iter()
        .filter(|order| page_order_rules.validate_print_order(order))
        .map(|order| order[order.len() / 2])
        .sum::<u32>() as i64;

    let result_b = print_orders
        .iter()
        .filter(|order| !page_order_rules.validate_print_order(order))
        .map(|order| {
            let mut order = order.clone();
            order.sort_by(|a, b| page_order_rules.ordering(*a, *b).unwrap());
            order
        })
        .map(|order| order[order.len() / 2])
        .sum::<u32>() as i64;

    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();
    assert_eq!(solution(input), (143, 123));
}

#[test]
fn test_parser() {
    assert_eq!(pair_of_ints("47|53").unwrap().1, (47, 53));
    let print_orders = "75,47,61,53,29
97,61,53,29,13";
    assert_eq!(
        print_orders_parser(print_orders).unwrap().1,
        vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]]
    );
    let input = "47|53
97|13

75,47,61,53,29
97,61,53,29,13";
    let mut parser = (
        page_order_rules_parser,
        preceded(many1(newline), print_orders_parser),
    );
    let result = parser.parse(input);
    assert!(result.is_ok());
}

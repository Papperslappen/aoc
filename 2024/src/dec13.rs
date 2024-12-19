use nom::{
    self, branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list1, sequence::Tuple, IResult, Parser,
};

fn posint(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse).parse(input)
}

fn button_parser(input: &str) -> IResult<&str, (i64, i64)> {
    let result = (
        tag("Button "),
        alt((tag("A"), tag("B"))),
        tag(": X+"),
        posint,
        tag(", Y+"),
        posint,
    )
        .parse(input)?;
    Ok((result.0, (result.1 .3, result.1 .5)))
}

fn target_parser(input: &str) -> IResult<&str, (i64, i64)> {
    let (rest, (_, x, _, y)) = (tag("Prize: X="), posint, tag(", Y="), posint).parse(input)?;
    Ok((rest, (x, y)))
}

fn machine_parser(input: &str) -> IResult<&str, Machine> {
    let (rest, (button_a, _, button_b, _, target)) = (
        button_parser,
        tag("\n"),
        button_parser,
        tag("\n"),
        target_parser,
    )
        .parse(input)?;
    Ok((
        rest,
        Machine {
            button_a,
            button_b,
            target,
        },
    ))
}

struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    target: (i64, i64),
}

impl Machine {
    fn lowest_cost(&self, max_presses: i64) -> Option<i64> {
        let a = self.button_a.0;
        let b = self.button_b.0;
        let c = self.button_a.1;
        let d = self.button_b.1;
        let det = a * d - b * c;
        if self.button_a == (0, 0) || self.button_b == (0, 0) || det == 0 {
            panic!("Linear dependence. Lets hope we don't have to cover this case")
        } else {
            let adj_times_target = (
                d * self.target.0 - b * self.target.1,
                -c * self.target.0 + a * self.target.1,
            );
            if adj_times_target.0 % det == 0 && adj_times_target.1 % det == 0 {
                let x = adj_times_target.0 / det;
                let y = adj_times_target.1 / det;
                let cost = 3 * x + y;
                println!("Solved: A = {}, B = {}, Cost = {}", x, y, cost);
                if (0..=max_presses).contains(&x) && (0..=max_presses).contains(&y) {
                    Some(cost)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let input = input.join("\n");
    let (_, machines) = separated_list1(tag("\n\n"), machine_parser)
        .parse(&input)
        .unwrap();
    let result_a = machines
        .iter()
        .filter_map(|machine| machine.lowest_cost(100))
        .sum();
    let result_b = machines
        .iter()
        .map(|machine| Machine {
            button_a: machine.button_a,
            button_b: machine.button_b,
            target: (
                machine.target.0 + 10000000000000,
                machine.target.1 + 10000000000000,
            ),
        })
        .filter_map(|machine| machine.lowest_cost(i64::MAX))
        .sum();
    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert_eq!(solution(input).0, 480);
}

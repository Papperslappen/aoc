use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    IResult, Parser,
};

#[derive(Debug)]
struct Mul {
    first: i64,
    second: i64,
}

impl Mul {
    fn result(&self) -> i64 {
        self.first * self.second
    }
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(Mul),
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn three_digit_int(input: &str) -> IResult<&str, i64> {
    map_res(take_while_m_n(1, 3, is_digit), str::parse).parse(input)
}

fn mul(input: &str) -> IResult<&str, Mul> {
    let (input, (_, first, _, second, _)) = (
        tag("mul("),
        three_digit_int,
        tag(","),
        three_digit_int,
        tag(")"),
    )
        .parse(input)?;
    Ok((input, Mul { first, second }))
}

fn mul_instruction(input: &str) -> IResult<&str, Instruction> {
    mul.map(Instruction::Mul).parse(input)
}

fn dont_intruction(input: &str) -> IResult<&str, Instruction> {
    tag("don't()").map(|_| Instruction::Dont).parse(input)
}

fn do_instruction(input: &str) -> IResult<&str, Instruction> {
    tag("do()").map(|_| Instruction::Do).parse(input)
}

fn all_muls(input: &str) -> IResult<&str, Vec<Mul>> {
    let mut matches = vec![];
    for start in 0..input.len() {
        match mul(&input[start..]) {
            Ok((_, mul)) => {
                matches.push(mul);
            }
            Err(_) => { //no match, go on
            }
        }
    }

    Ok((input, matches))
}

fn all_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut matches = vec![];
    for start in 0..input.len() {
        match alt((mul_instruction, do_instruction, dont_intruction)).parse(&input[start..]) {
            Ok((_, mul)) => {
                matches.push(mul);
            }
            Err(_) => { //no match, go on
            }
        }
    }

    Ok((input, matches))
}

pub(crate) fn solution(read_input: Vec<String>) -> (i64, i64) {
    let input = read_input.join("\n");
    let result_a = all_muls(&input).unwrap().1.iter().map(|m| m.result()).sum();
    let result_b = all_instructions(&input)
        .unwrap()
        .1
        .iter()
        .fold((true, 0), |(do_val, sum), inst| match inst {
            Instruction::Do => (true, sum),
            Instruction::Dont => (false, sum),
            Instruction::Mul(mul) => {
                if do_val {
                    (do_val, sum + mul.result())
                } else {
                    (do_val, sum)
                }
            }
        })
        .1;

    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(solution(vec![input.to_string()]).0, 161);
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(solution(vec![input.to_string()]).1, 48);
}

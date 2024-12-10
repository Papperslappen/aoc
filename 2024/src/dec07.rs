use nom::{
    self, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list1, sequence::separated_pair, IResult, Parser,
};

fn posint(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse).parse(input)
}

fn check_add_mul(target:i64,numbers:&Vec<i64>)->bool{
    let mut numbers = numbers.to_owned();
    numbers.reverse();

    let mut stack: Vec<(i64,&[i64])> = vec![(target,&numbers)];
    while let Some((remain,numbers))=stack.pop(){
        if remain == 0 && numbers.is_empty() {
            return true
        }
        if remain > 0 {
            if let Some(next) = numbers.first(){
                stack.push((remain-*next,&numbers[1..]));
                if remain % next == 0{
                    stack.push((remain/next,&numbers[1..]));
                }
            }

        } 
    }
    false
}

fn check_add_mul_concatenate(target:i64, numbers:&Vec<i64>)->bool{
    let mut numbers = numbers.to_owned();
    numbers.reverse();

    let mut stack: Vec<(i64,&[i64])> = vec![(target,&numbers)];
    while let Some((remain,numbers))=stack.pop(){
        if remain == 0 && numbers.is_empty() {
            return true
        }
        if remain > 0 {
            if let Some(next) = numbers.first(){
                stack.push((remain-*next,&numbers[1..]));
                if remain % next == 0{
                    stack.push((remain/next,&numbers[1..]));
                }
                let remainstring = remain.to_string();
                if remainstring.ends_with(&next.to_string()) && remainstring.len()>next.to_string().len(){ 
                   let new_number = remainstring.strip_suffix(&next.to_string()).unwrap().parse().unwrap();
                   stack.push((new_number,&numbers[1..]));
                }
            }

        } 
    }
    false
}

pub(crate) fn solution(read_input: Vec<String>) -> (i64, i64) {
    let result_a = read_input
        .iter()
        .map(|s| {
            let mut parser = separated_pair(posint, tag(": "), separated_list1(tag(" "), posint));
            parser.parse(s).unwrap().1
        }).filter(|(target, numbers)|{
            check_add_mul(*target,numbers)
        }).map(|(target,_)| target).sum();

    let result_b = read_input
    .iter()
    .map(|s| {
        let mut parser = separated_pair(posint, tag(": "), separated_list1(tag(" "), posint));
        parser.parse(s).unwrap().1
    }).filter(|(target, numbers)|{
        check_add_mul_concatenate(*target,numbers)
    }).map(|(target,_)| target).sum::<i64>();
    (result_a,result_b)
}

#[test]
fn test_solution(){
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"        .lines()
.map(|l| l.to_string())
.collect::<Vec<_>>();

assert_eq!(solution(input),(3749,11387));

}

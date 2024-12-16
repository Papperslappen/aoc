use std::collections::HashMap;

fn split(v: u64) -> (u64, u64) {
    let nd = (v as f64).log10() as usize + 1;
    let p = 10_u32.pow(nd as u32 / 2) as u64;
    let l = v / p;
    let r = v % p;
    (l, r)
}

fn numberwang(number: u64) -> Vec<u64> {
    if number == 0 {
        vec![1]
    } else if number.to_string().len() % 2 == 0 {
        let (num1,num2)=split(number);
        vec![num1, num2]
    } else {
        vec![number * 2024]
    }
}

fn forward_blink(stone: u64, blinks: usize, memory: &mut HashMap<(u64, usize), usize>) -> Vec<u64> {
    (0..blinks).fold(vec![stone], |vec, blinks| {
        let new_vec: Vec<u64> = vec.into_iter().flat_map(numberwang).collect();
        memory.insert((stone, blinks + 1), new_vec.len());
        new_vec
    })
}

fn backward_blink(stones: Vec<u64>, blinks: usize) -> usize {
    let mut stones = stones
        .into_iter()
        .map(|stone| (stone, blinks))
        .collect::<Vec<_>>();
    let mut memory = HashMap::<(u64, usize), usize>::new();
    let mut sum = 0;
    while let Some((stone, blinks_left)) = stones.pop() {
        if let Some(stone_count) = memory.get(&(stone, blinks_left)) {
            sum += stone_count;
        } else if blinks_left == 0 {
            sum += 1;
        } else {
            let blinks = blinks_left.min(5);
            let new_stones = forward_blink(stone,blinks, &mut memory);
            if new_stones.iter().all(|stone| memory.contains_key(&(*stone,blinks_left-blinks))){
                let score = new_stones.into_iter().map(|stone|memory.get(&(stone,blinks_left-blinks)).unwrap()).sum();
                sum += score;
                memory.insert((stone,blinks_left),score);
            }else{
                new_stones.into_iter().for_each(|stone| stones.push((stone,blinks_left-blinks)));
            }
        }
    }
    println!("memory size: {}",memory.len());
    sum
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let input = &input[0];
    let stones: Vec<u64> = input.split(' ').map(|s| s.parse().unwrap()).collect();

    let result_a = backward_blink(stones.clone(), 25) as i64;
    let result_b = backward_blink(stones, 75) as i64;
    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "125 17".lines().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(solution(input).0, 55312);
}

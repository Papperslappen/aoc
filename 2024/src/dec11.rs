use std::collections::HashMap;

fn numberwang(number: u64) -> Vec<u64> {
    if number == 0 {
        vec![1]
    } else if number.to_string().len() % 2 == 0 {
        let num = number.to_string();
        let len = num.len();
        let (num1, num2) = num.split_at(len / 2);
        vec![num1.parse().unwrap(), num2.parse().unwrap()]
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
            println!("cache hit! stone: {} blinks left: {}", stone, blinks_left);
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

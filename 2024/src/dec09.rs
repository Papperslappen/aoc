// I'm not proud of this one
use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Space {
    File(usize),
    Empty,
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let input: Vec<i64> = input[0]
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let filesystem = input.iter().chunks(2).into_iter().enumerate().fold(
        Vec::<(Space, usize)>::new(),
        |mut filesystem, (id, mut chunk)| {
            filesystem.push((Space::File(id), *chunk.next().unwrap() as usize));
            if let Some(space) = chunk.next() {
                filesystem.push((Space::Empty, *space as usize));
            }
            filesystem
        },
    );

    let filesystem: VecDeque<_> = filesystem
        .into_iter()
        .filter(|(_, size)| *size > 0)
        .collect();
    let solution_a: usize = (0..)
        .scan(filesystem.clone(), |filesystem, space_number| {
            use Space::*;
            match filesystem.pop_front()? {
                (File(id), size) => {
                    if size > 1 {
                        filesystem.push_front((File(id), size - 1));
                    }
                    Some(id * space_number)
                }
                (Empty, size) => {
                    while filesystem.back()?.0 == Empty {
                        filesystem.pop_back()?;
                    }
                    let return_value;
                    if let (File(id), size) = filesystem.pop_back()? {
                        if size > 1 {
                            filesystem.push_back((File(id), size - 1));
                        }
                        return_value = id * space_number;
                    } else {
                        panic!("AARGH")
                    }
                    if size > 1 {
                        filesystem.push_front((Empty, size - 1));
                    }
                    Some(return_value)
                }
            }
        })
        .sum();

    // part b
    let fs_defrag =
        filesystem
            .iter()
            .rev()
            .fold(filesystem.clone(), |mut fs, (space, file_size)| {
                if matches!(space, Space::File(_)) {
                    let (index, _) = fs
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, (find_space, _))| find_space == space)
                        .unwrap();
                    if let Some((empty_index, (_, empty_size))) = fs
                        .iter()
                        .enumerate()
                        .find(|(at_index, (space, empty_size))| {
                            matches!(space, Space::Empty)
                                && at_index < &index
                                && empty_size >= file_size
                        })
                        .map(|(a, (b, c))| (a, (b.clone(), *c)))
                    //UGH
                    {
                        let (old_space_index, (_, old_size)) = fs
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_, (old_space, _))| old_space == space)
                            .unwrap();
                        fs[old_space_index] = (Space::Empty, *old_size);
                        fs[empty_index] = (space.clone(), *file_size);
                        if empty_size > *file_size {
                            fs.insert(empty_index + 1, (Space::Empty, empty_size - file_size))
                        }
                    }
                }
                fs
            });
    let solution_b: usize = (0..)
        .scan(fs_defrag, |fs, i| {
            use Space::*;
            match fs.pop_front()? {
                (File(id), size) => {
                    if size > 1 {
                        fs.push_front((File(id), size - 1));
                    }
                    Some(id * i)
                }
                (Empty, size) => {
                    if size > 1 {
                        fs.push_front((Empty, size - 1));
                    }
                    Some(0)
                }
            }
        })
        .sum();

    (solution_a as i64, solution_b as i64)
}

#[test]
fn test_solution() {
    let input = vec!["2333133121414131402".to_string()];

    assert_eq!(solution(input), (1928, 2858));
}

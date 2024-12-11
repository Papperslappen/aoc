use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Map {
    fn inside(&self, (col, row): (usize, usize)) -> bool {
        col < self.width && row < self.height
    }
    fn count_nodes_inside(&self) -> i64 {
        let nodes = self
            .antennas
            .iter()
            .flat_map(|(_, antennas)| {
                antennas.iter().permutations(2).map(|a| {
                    let first = (a[0].0 as i64, a[0].1 as i64);
                    let second = (a[1].0 as i64, a[1].1 as i64);
                    let diff = (second.0 - first.0, second.1 - first.1);
                    (first.0 - diff.0, first.1 - diff.1)
                })
            })
            .filter(|node| {
                node.0 >= 0 && node.1 >= 0 && self.inside((node.0 as usize, node.1 as usize))
            })
            .collect::<HashSet<_>>();
        nodes.len() as i64
    }

    fn count_nodes_inside_include_harmonics(&self) -> i64 {
        let nodes = self
            .antennas
            .iter()
            .flat_map(|(_, antennas)| {
                antennas.iter().permutations(2).flat_map(|a| {
                    let first = (a[0].0 as i64, a[0].1 as i64);
                    let second = (a[1].0 as i64, a[1].1 as i64);
                    let diff = (second.0 - first.0, second.1 - first.1);
                    (0..)
                        .map(move |i| (i * diff.0, i * diff.1))
                        .map(move |step| (first.0 - step.0, first.1 - step.1))
                        .take_while(|node| {
                            node.0 >= 0
                                && node.1 >= 0
                                && self.inside((node.0 as usize, node.1 as usize))
                        })
                        .chain(
                            (0..)
                                .map(move |i| (i * diff.0, i * diff.1))
                                .map(move |step| (first.0 + step.0, first.1 + step.1))
                                .take_while(|node| {
                                    node.0 >= 0
                                        && node.1 >= 0
                                        && self.inside((node.0 as usize, node.1 as usize))
                                }),
                        )
                })
            })
            .collect::<HashSet<_>>();
        nodes.len() as i64
    }

    fn from_strings(strings: &[String]) -> Map {
        let width = strings[0].len();
        let height = strings.len();
        let antennas = strings
            .iter()
            .enumerate()
            .flat_map(|(row, s)| s.chars().enumerate().map(move |(col, c)| (c, (col, row))))
            .filter(|(c, _)| *c != '.')
            .fold(HashMap::new(), |mut antennas, (c, coordinate)| {
                antennas
                    .entry(c)
                    .and_modify(|e: &mut Vec<(usize, usize)>| e.push(coordinate))
                    .or_insert(vec![coordinate]);
                antennas
            });
        Map {
            width,
            height,
            antennas,
        }
    }
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let map = Map::from_strings(&input);
    let result_a = map.count_nodes_inside();
    let result_b = map.count_nodes_inside_include_harmonics();
    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();
    assert_eq!(solution(input), (14, 34))
}

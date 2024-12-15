use std::collections::HashSet;

struct HikeMap {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl HikeMap {
    fn from_lines(lines: Vec<String>) -> HikeMap {
        let width = lines[0].len();
        let height = lines.len();
        let data = lines
            .join("")
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        HikeMap {
            width,
            height,
            data,
        }
    }

    fn all_neighbors(&self, (col, row): (usize, usize)) -> Vec<(usize, usize)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(offset_col, offset_row)| {
                Some((
                    col.checked_add_signed(offset_col)?,
                    row.checked_add_signed(offset_row)?,
                ))
            })
            .filter(|coordinate| self.inside(*coordinate))
            .collect()
    }

    fn inside(&self, (col, row): (usize, usize)) -> bool {
        col < self.width && row < self.height
    }

    fn all_coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |row| (0..self.width).map(move |col| (col, row)))
    }

    fn height_at(&self, (col, row): (usize, usize)) -> Option<u8> {
        if self.inside((col, row)) {
            self.data.get(col + self.width * row).copied()
        } else {
            None
        }
    }

    fn trail_score(&self, (col, row): (usize, usize)) -> i64 {
        let mut stack = vec![(col, row)];
        let mut visited = HashSet::<(usize, usize)>::new();
        let mut score = 0;
        while let Some(coordinate) = stack.pop() {
            visited.insert(coordinate);
            let height = self.height_at(coordinate).unwrap();
            if height == 9 {
                score += 1;
            } else {
                self.all_neighbors(coordinate)
                    .iter()
                    .filter(|c| self.height_at(**c).unwrap() == height + 1 && !visited.contains(c))
                    .for_each(|c| {
                        stack.push(*c);
                    });
            }
        }
        score
    }

    fn trailcount(&self, (col, row): (usize, usize)) -> i64 {
        let mut stack = vec![(col, row)];
        let mut score = 0;
        while let Some(coordinate) = stack.pop() {
            let height = self.height_at(coordinate).unwrap();
            if height == 9 {
                score += 1;
            } else {
                self.all_neighbors(coordinate)
                    .iter()
                    .filter(|c| self.height_at(**c).unwrap() == height + 1)
                    .for_each(|c| {
                        stack.push(*c);
                    });
            }
        }
        score
    }

    fn all_trails_score(&self) -> i64 {
        self.all_coordinates()
            .filter(|c| self.height_at(*c).unwrap() == 0)
            .map(|c| self.trail_score(c))
            .sum()
    }
    fn all_trails_count(&self) -> i64 {
        self.all_coordinates()
            .filter(|c| self.height_at(*c).unwrap() == 0)
            .map(|c| self.trailcount(c))
            .sum()
    }
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let result_a = HikeMap::from_lines(input.clone()).all_trails_score();
    let result_b = HikeMap::from_lines(input).all_trails_count();

    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert_eq!(solution(input), (36, 81));
}

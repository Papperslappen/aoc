use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    E,
    N,
    W,
    S,
}

impl Direction {
    fn step(&self, size: usize, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        let newcol = match self {
            E => col.checked_add(size)?,
            W => col.checked_sub(size)?,
            _ => col,
        };
        let newrow = match self {
            N => row.checked_sub(size)?,
            S => row.checked_add(size)?,
            _ => row,
        };
        Some((newcol, newrow))
    }
    fn right_turn(&self) -> Direction {
        match self {
            Direction::E => Direction::S,
            Direction::N => Direction::E,
            Direction::W => Direction::N,
            Direction::S => Direction::W,
        }
    }
}
struct Map {
    width: usize,
    height: usize,
    obstacle_coordinates: HashSet<(usize, usize)>,
}

impl Map {
    fn inside(&self, (col, row): (usize, usize)) -> bool {
        col >= self.width || row >= self.height
    }

    fn walk(
        &self,
        (col, row): (usize, usize),
        direction: Direction,
        extra: Option<(usize, usize)>,
    ) -> Option<((usize, usize), Direction)> {
        let (new_col, new_row) = direction.step(1, (col, row))?;
        if self.inside((new_col, new_row)) {
            None
        } else if self.is_obstructed((new_col, new_row), extra) {
            Some(((col, row), direction.right_turn()))
        } else {
            Some(((new_col, new_row), direction))
        }
    }

    fn is_obstructed(&self, (col, row): (usize, usize), extra: Option<(usize, usize)>) -> bool {
        self.obstacle_coordinates.contains(&(col, row))
            || extra.map_or(false, |extra| extra == (col, row))
    }
}

fn make_map(strings: &[String]) -> (Map, (usize, usize, Direction)) {
    let height = strings.len();
    let width = strings[0].len();
    let non_empty = strings
        .iter()
        .enumerate()
        .flat_map(|(row, string)| {
            string
                .chars()
                .enumerate()
                .map(move |(col, c)| (col, row, c))
        })
        .filter(|(_, _, c)| *c != '.')
        .collect::<Vec<_>>();
    let guard = non_empty
        .iter()
        .find(|(_, _, c)| *c == '^')
        .map(|(col, row, _)| (*col, *row, Direction::N))
        .unwrap();
    let obstacle_coordinates = non_empty
        .into_iter()
        .filter(|(_, _, c)| *c == '#')
        .map(|(col, row, _)| (col, row))
        .collect();

    (
        Map {
            width,
            height,
            obstacle_coordinates,
        },
        guard,
    )
}

pub(crate) fn solution(read_input: Vec<String>) -> (i64, i64) {
    let (map, (start_col, start_row, start_direction)) = make_map(&read_input);
    let mut visited = HashSet::new();
    visited.insert((start_col, start_row));

    let mut col = start_col;
    let mut row = start_row;
    let mut direction = start_direction;
    while let Some(((new_col, new_row), new_direction)) = map.walk((col, row), direction, None) {
        visited.insert((new_col, new_row));
        col = new_col;
        row = new_row;
        direction = new_direction;
    }
    let result_a = visited.len() as i64;
    let result_b = visited
        .iter()
        .filter(|candidate| candidate != &&(start_col, start_row))
        .filter(|(candidate_col, candidate_row)| {
            let mut visited = HashSet::new();
            visited.insert((start_col, start_row, start_direction));
            let mut col = start_col;
            let mut row = start_row;
            let mut direction = start_direction;
            while let Some(((new_col, new_row), new_direction)) = map.walk(
                (col, row),
                direction,
                Some((*candidate_col, *candidate_row)),
            ) {
                if visited.contains(&(new_col, new_row, new_direction)) {
                    return true;
                }
                visited.insert((new_col, new_row, new_direction));
                col = new_col;
                row = new_row;
                direction = new_direction;
            }
            false
        })
        .count() as i64;

    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    assert_eq!(solution(input), (41, 6));
}

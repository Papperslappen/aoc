#[derive(Clone, Copy, Debug)]
enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
}
impl Direction {
    fn step(&self, size: usize, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        let newcol = match self {
            E | NE | SE => col.checked_add(size)?,
            W | NW | SW => col.checked_sub(size)?,
            _ => col,
        };
        let newrow = match self {
            N | NW | NE => row.checked_sub(size)?,
            S | SW | SE => row.checked_add(size)?,
            _ => row,
        };
        Some((newcol, newrow))
    }
}

struct Array2d {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Array2d {
    fn from_lines(lines: Vec<String>) -> Array2d {
        let width = lines[0].len();
        let height = lines.len();
        let data = lines.join("").chars().collect();
        Array2d {
            width,
            height,
            data,
        }
    }

    fn inside(&self, (col, row): (usize, usize)) -> bool {
        col >= self.width || row >= self.height
    }

    fn char_at(&self, (col, row): (usize, usize)) -> Option<char> {
        if self.inside((col, row)) {
            None
        } else {
            self.data.get(col + self.width * row).map(|c| c.to_owned())
        }
    }
    fn all_coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |row| (0..self.width).map(move |col| (col, row)))
    }

    fn find_word(&self, word: &str, at: (usize, usize), direction: Direction) -> bool {
        let wordlen = word.len();
        word.chars()
            .zip((0..wordlen).map(|step| direction.step(step, at).and_then(|at| self.char_at(at))))
            .all(|(c1, c2)| c2.map_or(false, |c2| c2 == c1))
    }
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let array = Array2d::from_lines(input);
    use Direction::*;

    let result_a = array
        .all_coordinates()
        .flat_map(|coordinate| {
            let coordinate = &coordinate;
            let array = &array;
            [E, NE, N, NW, W, SW, S, SE]
                .into_iter()
                .map(move |direction| array.find_word("XMAS", *coordinate, direction))
                .collect::<Vec<_>>()
        })
        .filter(|f| *f)
        .count() as i64;

    let result_b = array
        .all_coordinates()
        .filter(|(col, row)| {
            (array.find_word("MAS", (*col, *row), SE) || array.find_word("SAM", (*col, *row), SE))
                && (array.find_word("MAS", (*col + 2, *row), SW)
                    || array.find_word("SAM", (*col + 2, *row), SW))
        })
        .count() as i64;

    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        .lines()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(solution(input), (18, 9));
}

#[test]
fn test_find_word() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        .lines()
        .map(|s| s.to_string())
        .collect();

    let array = Array2d::from_lines(input);
    assert_eq!(array.char_at((0, 0)), Some('M'));
    assert!(array.find_word("XMAS", (0, 4), Direction::E));

    let input = "XM
AS"
    .lines()
    .map(|s| s.to_string())
    .collect();
    let array = Array2d::from_lines(input);
    assert!(!array.find_word("XMAS", (0, 0), Direction::E));
}

#[test]
fn test_direction() {
    use Direction::*;
    let p = (100, 100);
    assert_eq!(N.step(1, p), Some((100, 99)));
    assert_eq!(S.step(1, p), Some((100, 101)));
    assert_eq!(W.step(1, p), Some((99, 100)));
    assert_eq!(E.step(1, p), Some((101, 100)));
    assert_eq!(NE.step(1, p), Some((101, 99)));
    assert_eq!(SE.step(1, p), Some((101, 101)));
    assert_eq!(NW.step(1, p), Some((99, 99)));
    assert_eq!(SW.step(1, p), Some((99, 101)));
}

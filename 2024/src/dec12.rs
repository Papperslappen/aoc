use std::collections::HashSet;

struct Garden {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Garden {
    fn from_lines(lines: Vec<String>) -> Garden {
        let width = lines[0].len();
        let height = lines.len();
        let data = lines.join("").chars().collect();
        Garden {
            width,
            height,
            data,
        }
    }
    fn inside(&self, (col, row): &(usize, usize)) -> bool {
        *col < self.width && *row < self.height
    }

    fn all_coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |row| (0..self.width).map(move |col| (col, row)))
    }

    fn flower_at(&self, (col, row): &(usize, usize)) -> Option<char> {
        if self.inside(&(*col, *row)) {
            self.data.get(col + self.width * row).copied()
        } else {
            None
        }
    }

    fn all_neighbors(
        &self,
        (col, row): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(offset_col, offset_row)| {
                Some((
                    col.checked_add_signed(offset_col)?,
                    row.checked_add_signed(offset_row)?,
                ))
            })
            .filter(|coordinate| self.inside(coordinate))
    }

    fn cluster_at(&self, coordinate: &(usize, usize)) -> HashSet<(usize, usize)> {
        let mut stack = vec![*coordinate];
        let flower = self.flower_at(coordinate);
        let mut cluster = HashSet::new();
        while let Some(coordinate) = stack.pop() {
            cluster.insert(coordinate);
            self.all_neighbors(coordinate)
                .filter(|neighbor| {
                    self.flower_at(neighbor) == flower && !cluster.contains(neighbor)
                })
                .for_each(|c| stack.push(c));
        }
        cluster
    }

    fn all_clusters(&self) -> Vec<HashSet<(usize, usize)>> {
        let mut all_visited = HashSet::new();
        let mut all_clusters = Vec::new();
        for coordinate in self.all_coordinates() {
            if !all_visited.contains(&coordinate) {
                let cluster = self.cluster_at(&coordinate);
                cluster.iter().cloned().for_each(|c| {
                    all_visited.insert(c);
                });
                all_clusters.push(cluster);
            }
        }
        all_clusters
    }
}

fn area(cluster: &HashSet<(usize, usize)>) -> usize {
    cluster.len()
}

fn perimiter(cluster: &HashSet<(usize, usize)>) -> usize {
    cluster
        .iter()
        .map(|(col, row)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(move |(offset_col, offset_row)| {
                    (*col as isize + offset_col, *row as isize + offset_row)
                })
                .filter(|(col, row)| {
                    (*col < 0 || *row < 0) || !cluster.contains(&(*col as usize, *row as usize))
                })
                .count()
        })
        .sum()
}

fn isize_in_cluster(cluster: &HashSet<(usize, usize)>, (col, row): &(isize, isize)) -> bool {
    !((*col < 0 || *row < 0) || !cluster.contains(&(*col as usize, *row as usize)))
}

fn convex_corner_count(cluster: &HashSet<(usize, usize)>, (col, row): &(usize, usize)) -> usize {
    [
        [(-1, 0), (0, -1)],
        [(-1, 0), (0, 1)],
        [(1, 0), (0, -1)],
        [(1, 0), (0, 1)],
    ]
    .iter()
    .filter(|check| {
        check
            .iter()
            .map(|(offset_col, offset_row)| {
                (*col as isize + offset_col, *row as isize + offset_row)
            })
            .all(|c| !isize_in_cluster(cluster, &c))
    })
    .count()
}

fn concave_corner_count(cluster: &HashSet<(usize, usize)>, (col, row): &(usize, usize)) -> usize {
    [
        [(-1, 0), (0, -1)],
        [(-1, 0), (0, 1)],
        [(1, 0), (0, -1)],
        [(1, 0), (0, 1)],
    ]
    .iter()
    .filter(|check| {
        !isize_in_cluster(
            cluster,
            &(*col as isize + check[0].0, *row as isize + check[1].1),
        ) && check
            .iter()
            .map(|(offset_col, offset_row)| {
                (*col as isize + offset_col, *row as isize + offset_row)
            })
            .all(|c| isize_in_cluster(cluster, &c))
    })
    .count()
}

fn edges(cluster: &HashSet<(usize, usize)>) -> usize {
    cluster
        .iter()
        .map(|coordinate| {
            convex_corner_count(cluster, coordinate) + concave_corner_count(cluster, coordinate)
        })
        .sum()
}

pub(crate) fn solution(input: Vec<String>) -> (i64, i64) {
    let garden = Garden::from_lines(input);
    let result_a = garden
        .all_clusters()
        .iter()
        .map(|cluster| {
            // println!("area: {} perimiter: {}", area(cluster), perimiter(cluster));
            area(cluster) * perimiter(cluster)
        })
        .sum::<usize>() as i64;
    let result_b = garden
        .all_clusters()
        .iter()
        .map(|cluster| {
            // println!("area: {} perimiter: {}", area(cluster), perimiter(cluster));
            area(cluster) * edges(cluster)
        })
        .sum::<usize>() as i64;
    (result_a, result_b)
}

#[test]
fn test_solution() {
    let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert_eq!(solution(input).0, 772);

    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    assert_eq!(solution(input), (1930, 1206));
}

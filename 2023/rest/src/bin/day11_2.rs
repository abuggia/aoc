//use rest::*;
use std::collections::HashSet;
use itertools::Itertools;

type Pos = (isize, isize);

fn main() {
    let a = ans(include_str!("../../input11.txt"), 1000000);
    println!("ans: {a}");
}

fn ans(s: &str, offset: usize) -> isize {
    let mut total_cols = 0;
    let mut occupied_cols: HashSet<isize> = HashSet::new();
    let mut total_rows = 0;
    let mut occupied_rows: HashSet<isize> = HashSet::new();
    let mut galaxies: Vec<Pos> = vec![];

    for (y, line) in s.lines().enumerate() {
        let y = y as isize;
        total_rows += 1;
        for (x, char) in line.chars().enumerate() {
            let x = x as isize;
            total_cols += 1;
            if char == '#' {
                galaxies.push((x, y));
                occupied_cols.insert(x);
                occupied_rows.insert(y);
            }
        }
    }

    fn offsets(total: isize, occupied: HashSet<isize>, offset: usize) -> Vec<isize> {
        let (_, offsets): (isize, Vec<isize>) = (0..total)
            .fold((0, vec![]), |(mut curr, mut offsets), n| {
                if !occupied.contains(&n) {
                    curr += offset as isize
                }
                offsets.push(curr);
                (curr, offsets)
            });
        offsets
    }

    let xo = offsets(total_cols, occupied_cols, offset);
    let yo = offsets(total_rows, occupied_rows, offset);
    let translate = |(x,y): Pos| (x+xo.get(x as usize).unwrap(), y+yo.get(y as usize).unwrap());

    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(from, to)| (translate(from), translate(to)))
        .fold(0, |acc, ((x1,y1), (x2,y2)): (Pos, Pos)| acc + (x2-x1).abs() + (y2-y1).abs())
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc!{"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"};

    #[test]
    fn test1() {
        let a = ans(INPUT, 1);
        assert_eq!(a, 374);
    }

    #[test]
    fn test2() {
        let a = ans(INPUT, 10);
        assert_eq!(a, 1030);
    }

    #[test]
    fn test3() {
        let a = ans(INPUT, 100);
        assert_eq!(a, 8410);
    }
}


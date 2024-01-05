//use rest::*;
use std::collections::HashMap;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let a = ans(include_str!("../../input10.txt"));
    println!("ans: {a}");
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
enum Dir { N,S,E,W }

impl Dir {
    fn opp(self) -> Self {
        match self {
            N => S,
            E => W,
            S => N,
            W => E
        }
    }

    fn next_pos(&self, pos: &Pos) -> Pos {
        let mut pos = pos.clone();
        match self {
            N => pos.1 -= 1,
            E => pos.0 += 1,
            S => pos.1 += 1, 
            W => pos.0 -= 1
        }
        pos
    }
}

use Dir::*;

type Pos = (isize, isize);

#[derive(Debug)]
struct Nodes(HashMap<(Pos, Dir), Dir>);

impl Nodes {
    fn new() -> Self {
        Nodes(HashMap::new())
    }

    // 'L' => nodes.add(pos, N, E),
    fn add(&mut self, pos: Pos, dir1: Dir, dir2: Dir) {
        self.0.insert((pos, dir1), dir2);
        self.0.insert((pos, dir2), dir1);
    }

    // '|', 'N' -> is there a 'N'.next_dir(pos) with 'S'
    //
    fn next_dir(&self, pos: Pos, dir: Dir) -> Option<&Dir> {
        self.0.get(&(pos, dir.opp()))
    }

    fn start_dirs(&self, pos: &Pos) -> (Dir, Dir) {
        Dir::iter().fold(Vec::new(), |mut acc, dir| {
            if self.next_dir(dir.next_pos(&pos), dir).is_some() {
                acc.push(dir);
            }
            acc
        })
        .into_iter()
        .tuples()
        .next()
        .unwrap()
    }
}

fn ans(s: &str) -> isize {
    let (mut nodes, start): (Nodes, Option<Pos>) = s
        .lines()
        .enumerate()
        .fold((Nodes::new(), None), |(mut nodes, mut start), (y, line)| {
            for (x, char) in line.chars().enumerate() {
                let pos = (x as isize, y as isize);
                match char {
                    '|' => nodes.add(pos, N, S),
                    '-' => nodes.add(pos, E, W),
                    'L' => nodes.add(pos, N, E),
                    'J' => nodes.add(pos, N, W),
                    '7' => nodes.add(pos, S, W),
                    'F' => nodes.add(pos, S, E),
                    'S' => start = Some(pos),
                     _ => ()
                }
            }
            (nodes, start)
        });

    //dbg!(&nodes);
    let start_pos = start.unwrap();

    let mut count = 0;
    let (mut curr_dir, other_dir) = nodes.start_dirs(&start_pos);
    nodes.add(start_pos, curr_dir.clone(), other_dir);
    let mut curr_pos = curr_dir.next_pos(&start_pos);

    while {
        curr_pos = curr_dir.next_pos(&curr_pos);
        curr_dir = *nodes.next_dir(curr_pos, curr_dir).unwrap();
        count += 1;
        curr_pos != start_pos
    } {}

    (count as f32 / 2.0).ceil() as isize
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let a = ans(indoc!{"
.....
.S-7.
.|.|.
.L-J.
.....
"});
        assert_eq!(a, 4);
    }

    #[test]
    fn test2() {
        let a = ans(indoc!{"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"});
        assert_eq!(a, 8);
    }
}


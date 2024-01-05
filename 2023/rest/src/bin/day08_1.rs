use std::collections::BTreeMap;
use std::iter::repeat;

fn main() {
    let a = ans(include_str!("../../input08.txt"));
    println!("ans: {a}");
}

fn ans(s: &str) -> usize {
    let mut lines = s.lines();
    let moves = lines.next().unwrap();
    let mut nodes: BTreeMap<&str, (&str, &str)> = BTreeMap::new();

    lines.next();

    while let Some(line) = lines.next() {
        nodes.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    let mut curr = *nodes
        .first_key_value()
        .unwrap()
        .0;

    repeat(moves)
        .map(|move_| move_.chars())
        .flatten()
        .take_while(|move_| {
            let paths = nodes.get(curr).unwrap();
            curr = match move_ {
                'L' => paths.0,
                'R' => paths.1,
                _ => panic!("!")
            };
            curr != "ZZZ"
        })
        .count() + 1
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let a = ans(indoc!{"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"});
        assert_eq!(a, 2);
    }

    #[test]
    fn test2() {
        let a = ans(indoc!{"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"});
        assert_eq!(a, 6);
    }
}


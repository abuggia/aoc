use std::collections::BTreeMap;

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

    // 11678319315857
    nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|&k| {
            let mut curr = k;
            moves
                .chars()
                .cycle()
                .take_while(|move_| {
                    let paths = nodes.get(&curr).unwrap();
                    curr = match move_ {
                        'L' => paths.0,
                        'R' => paths.1,
                        _ => panic!("!")
                    };
                    !curr.ends_with("Z")
                })
                .count() + 1
        })
        .fold(1, |acc, num| lcm(acc, num))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let a = ans(indoc!{"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"});
        assert_eq!(a, 6);
    }
}


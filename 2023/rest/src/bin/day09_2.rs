use rest::*;
use itertools::Itertools;

fn main() {
    let a = ans(include_str!("../../input09.txt"));
    println!("ans: {a}");
}

//1969958987
fn ans(s: &str) -> isize {
    s.lines()
        .map(str::to_ints)
        .map(|mut curr| {
            let mut lasts: Vec<isize> = vec![];
            while curr.iter().any(|e| *e != 0) {
                lasts.push(*curr.first().unwrap());
                curr = curr
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
            }
            lasts
                .iter()
                .rev()
                .fold(0, |acc, e| e - acc)
        })
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test1() {
        let a = ans(indoc!{"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"});
        assert_eq!(a, 2);
    }
}


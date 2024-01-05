
use std::collections::HashMap;

fn main() {
    let a = ans(include_str!("../../input07.txt"));
    println!("ans: {a}");
}

const ORDER: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

fn secondary_order(hand: &str) -> String {
    hand.chars().map(|card| { 
        let order = ORDER.len() - ORDER.iter().position(|&c| c == card).unwrap();
        format!("{order:x}")
    })
    .collect()
}

fn strength(hand: &str) -> (usize, String) {
    let cmap: HashMap<char, usize> = hand
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            acc
                .entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            acc
        });

    let mut ctups: Vec<usize> = cmap
        .into_iter()
        .map(|(_, v)| v)
        .collect();
    ctups.sort_by(|a, b| b.cmp(a));

    let mut iter = ctups.into_iter();
    let first = iter.next().unwrap();
    let second = iter.next();

    let order = if first > 3 {
        first + 2
    } else if first == 3 {
        if second.unwrap() == 2 { 5 } else { 4 }
    } else if first == 2 {
        if second.unwrap() == 2 { 3 } else { 2 }
    } else {
        1
    };
    return (order, secondary_order(&hand))
}

fn ans(s: &str) -> usize {
    
    let mut m: Vec<((usize, String), usize)> = s.lines()
     .map(|line| {
         let parts: Vec<_> = line.split(" ").collect();
         (strength(parts[0]), parts[1].parse::<usize>().unwrap())
    })
    .collect();

    m.sort();
    //dbg!(&m);
    m.iter()
        .enumerate()
        .map(|(i, (_, bid))| (i+1)*bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_ints() {
        let a = ans(indoc!{"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"});
        assert_eq!(a, 6440);
    }
}


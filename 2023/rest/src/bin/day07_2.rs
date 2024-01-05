
use std::collections::BTreeMap;

fn main() {
    let a = ans(include_str!("../../input07.txt"));
    println!("ans: {a}");
}

const ORDER: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn secondary_order(hand: &str) -> String {
    hand.chars().map(|card| { 
        let order = ORDER.len() - ORDER.iter().position(|&c| c == card).unwrap();
        format!("{order:x}")
    })
    .collect()
}

fn card_counts(hand: &str) -> Vec<(usize, char)> {
    let cmap: BTreeMap<char, usize> = hand
        .chars()
        .filter(|&c| c != 'J')
        .fold(BTreeMap::new(), |mut acc, c| {
            acc
                .entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            acc
        });

    let mut ctups: Vec<(usize, char)> = cmap
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();

    ctups.sort_by(|a, b| b.cmp(a));
    ctups
}

fn strength(hand: &str) -> (usize, String) {
    let secondary_order = secondary_order(&hand); 
    let mut counts = card_counts(&hand).into_iter();
    // if not all Js
    if let Some((_, first_char)) = counts.next() {
        let to_replace = first_char.to_string();
        let hand = hand.replace('J', &to_replace);

        let mut counts = card_counts(&hand).into_iter();
        let (first_count, _) = counts.next().unwrap();
        let second = counts.next();

        let order = ORDER.len() + if first_count > 3 {
            first_count + 2
        } else if first_count == 3 {
            if second.unwrap().0 == 2 { 5 } else { 4 }
        } else if first_count  == 2 {
            if second.unwrap().0 == 2 { 3 } else { 2 }
        } else {
            let pos = ORDER.iter().position(|&c| c == hand.chars().next().unwrap()).unwrap();
            let ret = ORDER.len() - pos;
            return (ret, secondary_order)
        };
        (order, secondary_order)
    } else {
        (ORDER.len()+7, secondary_order)
    }
}

fn ans(s: &str) -> usize {
    
    let mut m: Vec<((usize, String), usize, String, usize)> = s.lines()
     .enumerate()
     .map(|(i, line)| {
         let parts: Vec<_> = line.split(" ").collect();
         (strength(parts[0]), i, parts[0].to_string(), parts[1].parse::<usize>().unwrap())
    })
    .collect();

    m.sort();
    m.iter()
        .enumerate()
        //.inspect(|(i, (s, index, hand, bid))|  println!("({index}) {hand} -> '{s:?}' ... {} * {bid} = {}", (i+1),(i+1)*bid))
        .map(|(i, (_, _, _, bid))| (i+1)*bid)
        .sum()
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn t1() {
        let a = ans(indoc!{"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"});
        assert_eq!(a, 5905);
    }

    #[test]
    fn t2() {
        let a = ans(indoc!{"
2345J 1
23456 2
"});
        assert_eq!(a, 4);
    }

    #[test]
    fn t3() {
        let a = ans(indoc!{"
AAJ332 1
AAA234 2
"});
        assert_eq!(a, 4);
    }

    #[test]
    fn t4() {
        let a = ans(indoc!{"
AAAAAA 1
AAAAAA 2
"});
        assert_eq!(a, 5);
    }


    #[test]
    fn t5() {
        let a = ans(indoc!{"
AAAAAA 1
AAAAAJ 2
"});
        assert_eq!(a, 4);
    }

    #[test]
    fn t6() {
        let a = ans(indoc!{"
2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41
"});
        assert_eq!(a, 6839);
    }
}




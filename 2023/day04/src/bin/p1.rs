use std::collections::HashSet;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, digit1},
    combinator::map_res,
    sequence::separated_pair,
    multi::separated_list1,
};

fn main() {
    let score = process(include_str!("../input1.txt"));
    let (_, score) = score.unwrap();
    println!("score: {score}");
}

fn process(s: &str) -> IResult<&str, i32> {
    let mut result = 0;

    for s in s.lines() {
        let (s, _) = tag("Card")(s)?;
        let (s, _) = space1(s)?;
        let (s, _) = digit1(s)?;
        let (s, _) = tag(":")(s)?;
        let (s, _) = space1(s)?;
        let (_, (ours, winning)) = cards(s)?;

        let ours: HashSet<i32> = ours.into_iter().collect();
        let winning: HashSet<i32> = winning.into_iter().collect();
        let num_matches = ours.intersection(&winning).collect::<Vec<_>>().len() as u32;
        result += if num_matches > 0 { 2_i32.pow(num_matches - 1) } else { 0 };
    }
    Ok((s, result))
}

fn cards(s: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> { 
    separated_pair(
        card,
        separated_pair(space1, tag("|"), space1),
        card
    )(s)
}

fn card(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, map_res(digit1, str::parse))(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let score = process("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");

        let (_, score) = score.unwrap();
        assert_eq!(13, score);
    }
}

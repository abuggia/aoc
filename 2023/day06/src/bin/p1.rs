use nom::{
    IResult,
    character::complete::{space1, digit1, newline},
    combinator::map_res,
    sequence::tuple,
    multi::separated_list1,
};

use nom_supreme::tag::complete::tag;

fn main() {
    let ans = process(include_str!("../../input1.txt"));
    let (_, ans) = ans.unwrap();
    println!("ans: {ans}");
}

fn process(s: &str) -> IResult<&str, i64> {
    let (s, _) = tuple((tag("Time:"), space1))(s)?;
    let (s, times) = separated_list1(space1, map_res(digit1, str::parse::<u64>))(s)?;
    let (s, _) = tuple((newline, tag("Distance:"), space1))(s)?;
    let (s, distances) = separated_list1(space1, map_res(digit1, str::parse::<u64>))(s)?;

    let ans: i64 = times
        .into_iter()
        .enumerate()
        .map(|(i, time)| {
            (0..time)
                .into_iter()
                .filter(|j| j * (time - j) > distances[i])
                .count() as i64
        })
        .product();

    Ok((s, ans as i64))
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    //#[test]
    #[allow(dead_code)]
    fn test_p1() {
        let input = indoc! {r#"
Time:      7  15   30
Distance:  9  40  200
        "#};

        let ans = process(input);
        let (_, res) = ans.unwrap();
        assert_eq!(288, res);
    }
}

use std::collections::BTreeMap;
use itertools::Itertools;
use std::ops::Range;

use nom::{
    IResult,
    character::complete::{alpha1, space1, digit1, newline},
    character::complete,
    combinator::map_res,
    sequence::{separated_pair, tuple},
    multi::separated_list1,
};

use nom_supreme::parser_ext::ParserExt;
use nom_supreme::tag::complete::tag;

fn main() {
    let ans = process(include_str!("../input1.txt"));
    let (_, location) = ans.unwrap();
    // 50780589 <-- too low
    println!("location: {location}");
}

#[derive(Debug)]
struct MapValues {
    dest_start: u64,
    source_range: Range<u64>,
}

impl MapValues {
    fn from_line(
        (dest_start, source_start, range_len): (u64, u64, u64)
        ) -> Result<Self, String> {
        let source_range = source_start..(source_start + range_len);
        Ok(MapValues { dest_start, source_range })
    }

    fn translate(&self, source_val: u64) -> Option<u64> {
        if self.source_range.contains(&source_val) {
            Some(self.dest_start + (source_val - self.source_range.start))
        } else {
            None
        }
    }
}

type Almanac = BTreeMap<(String, String), Vec<MapValues>>;

fn process(s: &str) -> IResult<&str, u64> {
    let (s, _) = tag("seeds: ")(s)?;
    let (s, mut seeds) = separated_list1(space1, map_res(digit1, str::parse::<u64>))(s)?;
    let (s, _) = tuple((newline, newline))(s)?;

    let (s, maps) = separated_list1(tuple((newline, newline)), map)(s)?;
    
    let almanac: Almanac = maps
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });

    let stages = vec!["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

    for (source, dest) in stages.into_iter().tuple_windows::<(&str, &str)>() {
        
        let map_values = almanac.get(&(source.to_string(), dest.to_string())).unwrap();

        for i in 0..seeds.len() {
            for mv in map_values {
                if let Some(val) = mv.translate(seeds[i]) {
                    seeds[i] = val;
                    break;
                }
            }
        }
    }

    //dbg!(almanac);
    //dbg!(&seeds);
    //println!(" here is the rest of s '{s}'");

    Ok((s, seeds.into_iter().min().unwrap()))
}

fn map(s: &str) -> IResult<&str, ((String, String), Vec<MapValues>)> {
    let (s, (source, dest)) = separated_pair(alpha1, tag("-to-"), alpha1)(s)?;
    let (s, _) = tuple((tag(" map:"), newline))(s)?;
    let (s, vals) = separated_list1(newline, map_res(map_line, MapValues::from_line))(s)?;
    Ok((s, ((source.to_string(), dest.to_string()), vals)))
}

fn map_line(s: &str) -> IResult<&str, (u64, u64, u64)> {
    tuple((
            complete::u64,
            complete::u64.preceded_by(tag(" ")),
            complete::u64.preceded_by(tag(" "))
    ))(s)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    //#[test]
    #[allow(dead_code)]
    fn test_p1() {
        let input = indoc! {r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#};

        let ans = process(input);
        let (_, location) = ans.unwrap();
        assert_eq!(35, location);
    }
}

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
    // 146006517 <- too high
    //  31161857
    println!("location: {location}");
}

#[derive(Debug, Clone)]
struct MapValues {
    dest_start: i64,
    source_range: Range<i64>,
}

impl MapValues {

    fn offset(&self) -> i64 {
        self.dest_start - self.source_range.start
    }

    fn from_line(
        (dest_start, source_start, range_len): (i64, i64, i64)
        ) -> Result<Self, String> {
        let source_range = source_start..(source_start + range_len);
        Ok(MapValues { dest_start, source_range })
    }
}

type Almanac = BTreeMap<(String, String), Vec<MapValues>>;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Clone)]
enum RangePart {
    StartRange,
    EndRange,
    StartMapping(i64),
    EndMapping,
}

use RangePart::*;

fn process(s: &str) -> IResult<&str, i64> {
    let (s, _) = tag("seeds: ")(s)?;
    let (s, seeds) = separated_list1(space1, map_res(digit1, str::parse::<i64>))(s)?;
    let (s, _) = tuple((newline, newline))(s)?;

    let (s, maps) = separated_list1(tuple((newline, newline)), map)(s)?;
    
    let almanac: Almanac = maps
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        });

    let stages = vec!["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

    let mut seeds: Vec<Vec<Range<i64>>> = seeds
        .into_iter()
        .tuples::<(i64, i64)>()
        .map(|(n, r)| vec![n..(n+r)] )
        .collect();

    //seeds.sort();
    //dbg!(&seeds);
    //
    //seed number 82 -> soil 84 -> fertilizer 84 -> water 84 -> light 77
    // -> temperature 45, humidity 46, location 46

    for (source, dest) in stages.into_iter().tuple_windows::<(&str, &str)>() {
        
        let map_values = almanac.get(&(source.to_string(), dest.to_string())).unwrap();
        //dbg!(map_values);
        let parts: Vec<(i64, RangePart)> = map_values
            .into_iter()
            .fold(vec![], |mut acc, mv| {
                acc.push((mv.source_range.start, StartMapping(mv.offset())));
                acc.push((mv.source_range.end - 1, EndMapping));
                acc
            });

        for i in 0..seeds.len() {
            let mut parts = parts.clone();
            for range in &seeds[i] {
                parts.push((range.start, StartRange));
                parts.push((range.end - 1, EndRange));
            }
            parts.sort();
            // if i == 0 {dbg!(&seeds[i]);}
            //dbg!(&parts);

            // todo: filter out map vals that have zero length? doesn't look like data has
            let mut ranges: Vec<Range<i64>> = vec![];
            let mut current_offset = 0;
            let mut current_range_start: Option<i64> = None;
            let mut range_depth = 0;
            //if i == 0 {println!("{source}-to-{dest} ... {:?}", map_values.clone().into_iter().map(|mv| mv.source_range).collect::<Vec<_>>());}
            for (j, part) in parts {
                //if i == 0 {println!(" - ({j}): {part:?}  (start: {current_range_start:?}, depth: {range_depth}, offset: {current_offset}");} 
                match part {
                    StartRange => {
                        if range_depth == 0 {
                            current_range_start = Some(j + current_offset);
                            range_depth += 1;
                        }
                    },
                    EndRange => {
                        if range_depth == 1 {
                            let start = current_range_start.take().unwrap();
                            ranges.push(start..(j+current_offset+1));
                            range_depth -= 1;
                        }
                    },
                    StartMapping(offset) => {
                        if let Some(start) = current_range_start.take() {
                            if j - start != 0 {
                                ranges.push(start..j);
                            }
                            current_range_start = Some(j+offset);
                        }
                        current_offset = offset;
                    },
                    EndMapping => {
                        if let Some(start) = current_range_start.take() {
                            ranges.push(start..(j+current_offset+1));
                            current_range_start = Some(j+1);
                        }
                        current_offset = 0;
                    }
                }
            }
                
            //if i == 1 { dbg!(&ranges); }

            seeds[i] = ranges;
            //if i == 1 {break 'outer;}
        }
    }

    Ok(
        (s,
         seeds
             .into_iter()
             .map(|v| v.into_iter().map(|r| r.start))
             .flatten()
             .min()
             .unwrap()
        )
    )
}

fn map(s: &str) -> IResult<&str, ((String, String), Vec<MapValues>)> {
    let (s, (source, dest)) = separated_pair(alpha1, tag("-to-"), alpha1)(s)?;
    let (s, _) = tuple((tag(" map:"), newline))(s)?;
    let (s, vals) = separated_list1(newline, map_res(map_line, MapValues::from_line))(s)?;
    Ok((s, ((source.to_string(), dest.to_string()), vals)))
}

fn map_line(s: &str) -> IResult<&str, (i64, i64, i64)> {
    tuple((
            complete::i64,
            complete::i64.preceded_by(tag(" ")),
            complete::i64.preceded_by(tag(" "))
    ))(s)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    #[test]
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
        assert_eq!(46, location);
    }
}

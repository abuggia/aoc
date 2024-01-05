
use std::collections::HashMap;

// 54980
fn main() {
    process(include_str!("../input1.txt"));
}

fn replace_nums(s: &str) -> Vec<char> {
    let word2num = HashMap::from([
        ("one", '1'),("two", '2'),("three", '3'),("four", '4'),("five", '5'),
        ("six", '6'),("seven", '7'),("eight", '8'),("nine", '9'),
    ]);
    let mut ret: Vec<char> = vec![];

    'outer: for i in 0..s.len() {
        let c = s[i..].chars().next().unwrap();

        if c.is_numeric() {
            ret.push(c);

        } else {

            for j in 2..=4 {
                if i >= j {
                    if let Some(c) = word2num.get(&s[i-j..=i]) {
                      ret.push(c.clone());
                      continue 'outer;
                    }
                }
            }

        }
    }

    ret
}

fn process(input: &str) {
    let sum = input
        .to_string()
        .lines()
        .map(|s| {
            let nums = replace_nums(s);
            if nums.len() > 0 {
                let first = nums.first().unwrap();
                let last = nums.last().unwrap();
                format!("{first}{last}").parse().unwrap()
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("=====");
    println!("{sum}");
    println!("=====");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let _ = process("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
");
        assert_eq!(1+1, 2);
    }

}

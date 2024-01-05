use rest::*;
use rayon::prelude::*;

// 7025
fn main() {
    let a = ans(include_str!("../../input12.txt"));
    println!("ans: {a}");
}

fn get_sizes(chars: Vec<char>) -> Vec<isize> {
    let mut chars = chars.into_iter().peekable();
    let mut ret = vec![];
    let mut damaged_count = 0;

    while let Some(char) = chars.next() {
        match char {
            '.' => {
                if damaged_count > 0 {
                    ret.push(damaged_count);
                    damaged_count = 0;
                }
            },
            '#' => damaged_count += 1,
            _ => panic!("no")
        }
        if chars.peek().is_none() && damaged_count > 0 {
            ret.push(damaged_count);
        }
    }
    ret
}

fn ans(input: &str) -> usize {

    input 
      .lines()
      .map(|line| {
          let mut parts = line.split(" ");
          let condition_records = parts.next().unwrap();
          let condition_records = [&*condition_records].repeat(4).join("?");

          let sizes = parts.next().unwrap().ints().repeat(5);
          /*
          let sizes = iter::repeat(sizes)
              .take(5)
              .flatten()
              .collect::<Vec<_>>();
              */
          //println!("{condition_records} {sizes:?}");
          //println!("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3");

          let unknowns_indices: Vec<isize> = condition_records
              .match_indices("?")
              .map(|t| t.0 as isize)
              .collect();

          (0..(2_usize.pow(unknowns_indices.len() as u32))).filter(|n| {
              let mut perm = format!("{:b}", n);
              let transformed: Vec<char> = condition_records
                  .chars()
                  .map(|c| {
                      match c {
                        '?' => match perm.pop() {
                            Some('1') => '#',
                            _ => '.'
                        }
                        c => c
                      }
                  })
                  .collect();

              sizes == get_sizes(transformed)
          })
          .count()
        })
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc!{"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"};

    //#[test]
    fn test1() {
        assert_eq!(4, ans(".??..??...?##. 1,1,3"));
    }

    //#[test]
    fn test2() {
        assert_eq!(1, ans("???.### 1,1,3"));
    }

    #[test]
    fn testall() {
        assert_eq!(525152, ans(INPUT));
    }
}


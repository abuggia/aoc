use anyhow::Result;

fn main() {
  let _ = process(include_str!("../input1.txt"));
}

fn process(input: &str) -> Result<()> {
    let sum = input
        .to_string()
        .lines()
        .fold(0, |acc, s| {
            let nums: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
            if nums.len() > 0 {
                let first = nums.first().unwrap();
                let last = nums.last().unwrap();
                let num: i32 = format!("{first}{last}").parse().unwrap();
                acc + num
            } else {
                acc
            }
        });

    println!("=====");
    println!("{sum}");
    println!("=====");
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    #[allow(dead_code)]
    fn test_input() {
        let _ = process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
");
        assert_eq!(1+1, 2);
    }

}

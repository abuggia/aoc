
fn main() {
    let ans = process(61709066 as i64, 643118413621041 as i64);
    println!("ans: {ans}");
}

fn process(time: i64, distance: i64) -> i64 {
    (0..time)
        .into_iter()
        .filter(|i| i * (time - i) > distance)
        .count() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(dead_code)]
    fn test_p1() {
        let ans = process(71530 as i64, 940200 as i64);
        assert_eq!(71503, ans);
    }
}

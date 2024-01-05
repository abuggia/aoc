#![feature(pattern)]

use std::str::pattern::Pattern;
use std::str::FromStr;

pub trait StrExt {
    fn ints_delim<'a, P: Pattern<'a>>(&'a self, del: P) -> Vec<isize>;
    fn ints(&self) -> Vec<isize>;
    fn ints_after<'a, P: Pattern<'a>>(&'a self, pat: P) -> Vec<isize>;
    //fn p(&self) -> Self;

    fn to_ints(s: &str) -> Vec<isize>{
        s.ints()
    }
}

impl StrExt for str {
    fn ints_delim<'a, P: Pattern<'a>>(&'a self, del: P) -> Vec<isize> {
        self
            .split(del)
            .map(isize::from_str)
            .map(|e| e.unwrap())
            .collect()
    }

    fn ints(&self) -> Vec<isize> {
        self.ints_delim(|e: char| { 
            e.is_whitespace() ||
            e == ','
        })
    }

    fn ints_after<'a, P: Pattern<'a>>(&'a self, pat: P) -> Vec<isize> {
        self
            .trim_start_matches(pat)
            .ints()
    }

    /*
    fn p(&self) -> Self {
        println!("{self}");
        &self
    }
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ints_del() {
        let v = "1|2|3".ints_delim("|");
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn test_ints() {
        let v = "1 2 3".ints();
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn test_ints_after() {
        let v = "here we have numbers: 1 2 3"
            .ints_after("here we have numbers: ");
        assert_eq!(v, [1, 2, 3]);
    }
}


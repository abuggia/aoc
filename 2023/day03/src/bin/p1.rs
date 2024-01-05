use std::ops::Deref;

fn main() {
    let res = process(include_str!("../input1.txt"));
    println!("{res}");
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {x, y}
    }

    fn is_adjacent(&self, other: &Position) -> bool {
        let x = self.x as i32;
        let y = self.y as i32;
        let o_x = other.x as i32;
        let o_y = other.y as i32;

        o_x == x - 1 && [y - 1, y, y+1].contains(&o_y) ||
        o_x == x && [y - 1, y+1].contains(&o_y) ||
        o_x == x + 1 && [y - 1, y, y+1].contains(&o_y)
    }
}

#[derive(Debug)]
struct Symbol(Position);

impl Deref for Symbol {
    type Target = Position;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Number {
    num: i32,
    positions: Vec<Position>
}

impl Number {
    fn is_adjacent(&self, symbols: &Vec<Symbol>) -> bool {
        symbols.iter().any(|s| self.positions.iter().any(|p| p.is_adjacent(&s.0)))
    }
}



fn process(input: &str) -> i32 {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut chars = line.chars().enumerate().peekable();

        while let Some((x, char)) = chars.next() {

            if char.is_numeric() {
                let mut num = char.to_string();
                let mut positions: Vec<Position> = vec![Position::new(x, y)];
                while let Some((x, digit)) = chars.next_if(|(_, c)| c.is_numeric()) {
                    num.push(digit);
                    positions.push(Position::new(x, y));
                }
                if let Ok(num) = num.parse::<i32>() {
                    numbers.push(Number { num, positions });
                }
            } else if char != '.' {
                symbols.push(Symbol(Position::new(x, y)));
            }
        }
    }

    numbers
        .into_iter()
        .filter(|n| n.is_adjacent(&symbols))
        .map(|n| n.num)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let symbols = process("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

        assert_eq!(symbols, 4361);
    }
}

use std::iter::Peekable;
use std::str::Chars;
use std::cmp;

fn main() {
    let games = p1(include_str!("../input1.txt"));
    let bag = ColorCounts{ blue: 14, green: 13, red: 12 };
    println!("part1: possible given bag: {}", games.sum_belong(&bag));
    println!("part2: sum of powers: {}", games.sum_of_powers());
}

#[derive(Debug)]
enum Color {
    Blue,Red,Green
}

impl From<String> for Color {
    fn from(s: String) -> Self {
        match s.as_str() {
            "blue" => Blue,
            "green" => Green,
            "red" => Red,
            _ => panic!("dunno what to do with {s}")
        }
    }
}
                        
use Color::*;

#[derive(Default, Debug)]
struct ColorCounts {
    blue: i32,
    red: i32,
    green: i32
}

impl ColorCounts {
    fn is_contained(&self, bag: &ColorCounts) -> bool {
        self.blue <= bag.blue &&
        self.green <= bag.green &&
        self.red <= bag.red
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    color_counts: ColorCounts,
}

impl Game {
    fn for_id(id: i32) -> Self {
        let color_counts: ColorCounts = Default::default();
        Game{ id, color_counts }
    }

    fn set_color_max(&mut self, color: Color, count: i32) {
        let cc = &mut self.color_counts;
        match color {
            Blue => cc.blue = cmp::max(cc.blue, count),
            Green => cc.green = cmp::max(cc.green, count),
            Red => cc.red = cmp::max(cc.red, count),
        }
    }

    fn is_possible(&self, bag: &ColorCounts) -> bool {
        self.color_counts.is_contained(bag)
    }

    fn power(&self) -> i32 {
        self.color_counts.blue * self.color_counts.green * self.color_counts.red
    }
}

struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Scanner<'a> {
        Scanner { chars: input.chars().peekable() }
    }

    fn take_game(&mut self) -> Option<i32> {
        self.take_str("Game ")?;
        let game_id = self.take_digits()?;
        self.skip();
        Some(game_id)
    }

    fn take_digits(&mut self) -> Option<i32> {
        let mut ret = String::from("");
        while let Some(n) = self.chars.next_if(|&c| c.is_numeric()) {
            ret.push(n);
        }
        ret.parse().ok()
    }

     //3 green, 4 blue, 1 red; 1
    fn take_color_count(&mut self) -> Option<(String, i32)> {
        let count = self.take_digits()?;
        self.skip();
        if let Some(c) = self.chars.peek() {
            let color = match c {
                'b' => self.take_str("blue")?,
                'g' => self.take_str("green")?,
                'r' => self.take_str("red")?,
                c => panic!("dunno why {c} showed up")
            };
            self.skip();
            return Some((color, count));
        }
        None
    }

    fn take_str(&mut self, s: &str) -> Option<String> {
        let res: String = self.chars.by_ref().take(s.len()).collect();
        if res == s {
            Some(res)
        } else {
            None
        }
    }

    fn skip(&mut self) {
        while let Some(_) = self
            .chars
            .next_if(|c| c.is_whitespace() || *c == ':' || *c == ';' || *c == ',') {
            continue
        }
    }
}

struct Games {
    games: Vec<Game>
}

impl Games {
    fn possible_games(&self, bag: &ColorCounts) -> Vec<&Game> {
        self
            .games
            .iter()
            .filter(|game| game.is_possible(&bag))
            .collect()
    }

    fn sum_belong(&self, bag: &ColorCounts) -> i32 {
        self
            .possible_games(bag)
            .iter()
            .map(|g| g.id)
            .sum()
    }

    fn sum_of_powers(&self) -> i32 {
        self
            .games
            .iter()
            .map(|game| game.power())
            .sum()
    }
}

fn p1(input: &str) -> Games {
    let games: Vec<Game> = input
        .lines()
        .filter_map(|line| {
            let mut scanner = Scanner::new(line);
            if let Some(game_id) = scanner.take_game() {
                let mut game = Game::for_id(game_id);
                while let Some((color, count)) = scanner.take_color_count() {
                   game.set_color_max(color.into(), count);
                }
                Some(game)
            } else {
                None
            }

        })
        .collect();

    Games{games}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let games = p1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        let bag = ColorCounts{ blue: 12 , green: 13, red: 14 };
        assert_eq!(8, games.sum_belong(&bag))
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
enum GameResult {
    Win,
    Lose,
    Draw
}

impl GameResult {
    fn from(v: char) -> GameResult {
        match v.to_ascii_uppercase() {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("Unsupported game result '{}'!", v)
        }
    }

    fn points(self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0
        }
    }
}

impl Hand {
    fn from(v: char) -> Hand {
        match v.to_ascii_uppercase() {
            'A' => Hand::Rock,
            'X' => Hand::Rock,
            'B' => Hand::Paper,
            'Y' => Hand::Paper,
            'C' => Hand::Scissors,
            'Z' => Hand::Scissors,
            _ => panic!("Unsupported hand '{}'!", v)
        }
    }

    fn defeated_by(self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock
        }
    }

    fn win_from(self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        }
    }

    fn points(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn play(self, other: Self) -> GameResult {
        // Note to self: https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html
        match self {
            _ if self == other => GameResult::Draw,
            _ if self.defeated_by() == other => GameResult::Lose,
            _ if other.defeated_by() == self => GameResult::Win,
            _ => panic!("Oops!")
        }
    }
}

pub fn exec(files_path: String) -> Result<(), Box<dyn std::error::Error>> {
    part_one(&files_path)?;
    part_two(&files_path)?;
    Ok(())
}

fn part_one(files_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(files_path.to_owned() + "/input").unwrap();
    let reader = BufReader::new(file);

    let score: u32 = reader.lines()
        .filter_map(|line| match line {
            Ok(line) => play(&line),
            _ => None
        })
        .sum();

    println!("Part one score is: {}", score);
    Ok(())
}

fn part_two(files_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(files_path.to_owned() + "/input").unwrap();
    let reader = BufReader::new(file);

    let score: u32 = reader.lines()
        .filter_map(|line| match line {
            Ok(line) => play2(&line),
            _ => None
        })
        .sum();

    println!("Part two score is: {}", score);
    Ok(())
}

fn play(game_match: &str) -> Option<u32> {
    let (opponent, myself) = game_match.trim().split_once(' ')?;

    let opponent = Hand::from(opponent.chars().nth(0)?);
    let myself = Hand::from(myself.chars().nth(0)?);
    let game_result = myself.play(opponent);

    Some(myself.points() + game_result.points())
}

fn play2(game_match: &str) -> Option<u32> {
    let (opponent, game_result) = game_match.trim().split_once(' ')?;

    let opponent = Hand::from(opponent.chars().nth(0)?);
    let game_result = GameResult::from(game_result.chars().nth(0)?);
    let myself = match game_result {
        GameResult::Draw => opponent,
        GameResult::Win => opponent.defeated_by(),
        GameResult::Lose => opponent.win_from()
    };

    Some(myself.points() + game_result.points())
}

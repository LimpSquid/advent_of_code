use crate::utils;
use std::io::BufRead;
use std::ops::{RangeInclusive};

trait RangeExt {
    fn contains(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
}

impl RangeExt for RangeInclusive<u8> {
    fn contains(&self, other: &Self) -> bool {
        other.contains(self.start()) &&
        other.contains(self.end())
    }

    fn intersects(&self, other: &Self) -> bool {
        other.contains(self.start()) ||
        other.contains(self.end())
    }
}

fn overlaps<T>(x: &T, y: &T) -> bool
where
    T: RangeExt
{
    x.contains(y) || y.contains(x)
}

fn intersects<T>(x: &T, y: &T) -> bool
where
    T: RangeExt
{
    x.intersects(y) || y.contains(x)
}

fn parse_range(pair: &str) -> Option<RangeInclusive<u8>> {
    let (x, y) = pair.split_once('-')?;
    let x: u8 = x.parse().ok()?;
    let y: u8 = y.parse().ok()?;
    Some(x ..= y)
}

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    part_one(files_dir.clone())?;
    part_two(files_dir.clone())?;
    Ok(())
}

fn part_one(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let count: u32 = reader.lines()
        .filter_map(|line| match line {
            Ok(line) => {
                let (x, y) = line.split_once(',')?;
                let x = parse_range(x)?;
                let y = parse_range(y)?;

                Some(overlaps(&x, &y) as u32)
            }
            _ => None
        })
        .sum();

    println!("Part one count is: {}", count);
    Ok(())
}
fn part_two(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let count: u32 = reader.lines()
        .filter_map(|line| match line {
            Ok(line) => {
                let (x, y) = line.split_once(',')?;
                let x = parse_range(x)?;
                let y = parse_range(y)?;

                Some(intersects(&x, &y) as u32)
            }
            _ => None
        })
        .sum();

    println!("Part two count is: {}", count);
    Ok(())
}

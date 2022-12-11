use crate::utils;
use std::{io::BufRead, collections::HashSet};

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    part_one(files_dir.clone())?;
    part_two(files_dir.clone())?;
    Ok(())
}

fn part_one(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let answer: usize = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let line = line.chars().collect::<Vec<_>>();
            let mut window = line.windows(4);
            window.position(|window| {
                let mut unique = HashSet::new();
                window.into_iter().all(move |x| unique.insert(x))
            })
        })
        .map(|pos| pos + 4)
        .sum();

    println!("Part one answer is: {}", answer);
    Ok(())
}
fn part_two(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let answer: usize = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let line = line.chars().collect::<Vec<_>>();
            let mut window = line.windows(14);
            window.position(|window| {
                let mut unique = HashSet::new();
                window.into_iter().all(move |x| unique.insert(x))
            })
        })
        .map(|pos| pos + 14)
        .sum();

    println!("Part two answer is: {}", answer);
    Ok(())
}

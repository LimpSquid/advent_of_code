use crate::utils;
use std::io::BufRead;

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    part_one(files_dir.clone())?;
    part_two(files_dir.clone())?;
    Ok(())
}

fn part_one(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;

    Ok(())
}
fn part_two(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;

    Ok(())
}

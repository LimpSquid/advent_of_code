use crate::utils;
use std::collections::HashSet;
use std::io::BufRead;

fn shared_char(mut vec: Vec<String>) -> Option<char> {
    let mut x: Vec<HashSet<char>> = Vec::new();
    let y = vec.pop()?;

    vec.into_iter().for_each(|s| x.push(s.chars().collect()));
    y.chars().find(|c| x.iter().all(|x| x.contains(c)))
}

struct Chunked<I>
where
    I: Iterator
{
    it: I
}

impl<I> Iterator for Chunked<I>
where
    I: Iterator
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        let mut chunk = Vec::new();
        for i in 0..3 {
            match self.it.next() {
                None if i == 0 => return None,
                None => return Some(chunk),
                Some(item) => chunk.push(item),
            }
        }
        Some(chunk)
    }
}

trait ChunkedIterator<I>
where
    I: Iterator
{
    fn chunked(self) -> Chunked<I>;
}

impl<I> ChunkedIterator<I> for I
where
    I: Iterator
{
    fn chunked(self) -> Chunked<I> {
        Chunked { it: self }
    }
}

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    part_one(files_dir.clone())?;
    part_two(files_dir.clone())?;
    Ok(())
}

fn part_one(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let sum: u32 = reader.lines()
        .filter_map(|line| match line {
            Ok(line) => {
                let (x, y) = line.split_at(line.len() / 2);
                shared_char(vec![String::from(x), String::from(y)])
            }
            _ => None
        })
        .map(|c| match c {
            lower if c >= 'a' && c <= 'z' => lower as u32 - 0x60,
            upper if c >= 'A' && c <= 'Z' => upper as u32 - 0x26,
            _ => panic!("Oops!")
        })
        .sum();
    println!("Part one sum of priorities is: {}", sum);

    Ok(())
}

fn part_two(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let sum: u32 = reader.lines()
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            _ => None
        })
        .chunked()
        .filter_map(|lines| shared_char(lines))
        .map(|c| match c {
            lower if c >= 'a' && c <= 'z' => lower as u32 - 0x60,
            upper if c >= 'A' && c <= 'Z' => upper as u32 - 0x26,
            _ => panic!("Oops!")
        })
        .sum();
    println!("Part two sum of group priorites is: {}", sum);

    Ok(())
}

use crate::utils;
use std::io::BufRead;
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Eq)]
struct Elve {
    num: i32,
    calories: i32
}

impl PartialOrd for Elve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialEq for Elve {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

impl Elve {
    fn add_calories(&mut self, calories: i32) {
        self.calories = self.calories + calories;
    }
}

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let mut elves: Vec<Elve> = vec![Elve::default()];

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            elves.push(Elve{
                num: elves.last().unwrap().num + 1,
                ..Default::default()
            });
        } else {
            elves.last_mut()
                .unwrap()
                .add_calories(line.parse()?);
        }
    }

    elves.sort();

    println!("Elve that is carrying the most calories: {:?}", elves.last());

    let sum_of_three: i32 = elves.into_iter()
        .rev()
        .take(3)
        .map(|x| x.calories)
        .sum();
    println!("Sum of calories from the top three elves: {}", sum_of_three);
    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Copy)]
struct Elve {
    num: i32,
    calories: i32
}

impl Elve {
    fn carrying_more_calories_than(self, other: Self) -> bool {
        self.calories > other.calories
    }

    fn add_calories(&mut self, calories: i32) {
        self.calories = self.calories + calories;
    }
}

pub fn exec(files_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(files_path + "/input").unwrap();
    let reader = BufReader::new(file);

    let mut elve = Elve::default();
    let mut curr_elve = Elve::default();

    for line in reader.lines() {
        let line = line?;

        let line = line.trim();
        if line.is_empty() {
            if curr_elve.carrying_more_calories_than(elve) {
                elve = curr_elve;
            }

            curr_elve = Elve {
                num: curr_elve.num + 1,
                ..Default::default()
            }
        } else {
            curr_elve.add_calories(line.parse()?);
        }
    }


    println!("Elve that is carrying the most calories: {:?}", elve);
    Ok(())
}

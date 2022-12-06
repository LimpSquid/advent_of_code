use crate::utils;
use std::io::{BufRead, Error, ErrorKind};

type Crate = char;
type CrateStack = Vec<Crate>;

#[derive(Debug, Default)]
struct Crane {
    stacks: [CrateStack; 9]
}

impl Crane {
    fn new() -> Crane {
        Crane::default()
    }

    fn apply_instr(&mut self, instr: &CraneInstruction) {
        assert!(instr.from < self.stacks.len());
        assert!(instr.to < self.stacks.len());

        if instr.from != instr.to {
            for _ in 0 .. instr.num_crates {
                let x = self.stacks[instr.from].pop().unwrap();
                self.stacks[instr.to].push(x);
            }
        }
    }
}

#[derive(Debug, Default)]
struct CraneInstruction {
    num_crates: usize,
    from: usize,
    to: usize,
}

impl CraneInstruction {
    fn parse(str: String) -> Result<CraneInstruction, Box<dyn std::error::Error>> {
        let split: Vec<&str> = str.split(' ').collect();
        match split[..] {
            [_, num, _, from, _, to] => {
                let instr = CraneInstruction {
                    num_crates: num.parse()?,
                    from: from.parse()?,
                    to: to.parse()?
                };
                Ok(instr)
            }
            _ => Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid crane instruction '{}'", str))))
        }
    }
}

pub fn solve(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    part_one(files_dir.clone())?;
    part_two(files_dir.clone())?;
    Ok(())
}

fn part_one(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;
    let mut crane = Crane::new();

    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| CraneInstruction::parse(line).ok())
        .for_each(|instr| crane.apply_instr(&instr));

    println!("Part one: {:?}", crane);
    Ok(())
}
fn part_two(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let reader = utils::input_file_reader(files_dir)?;

    Ok(())
}

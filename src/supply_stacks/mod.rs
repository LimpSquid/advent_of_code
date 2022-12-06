use crate::utils;
use core::panic;
use std::io::{Error, ErrorKind, Read};

type Crate = char;
type CrateStack = Vec<Crate>;

#[derive(Debug, Default)]
struct Crane {
    stacks: [CrateStack; 9]
}

impl Crane {
    fn parse(str: &str) -> Result<Crane, String> {
        let mut crane = Crane::default();
        let mut stacks: Vec<&str> = str.split("\n").collect();

        match stacks.pop() {
            Some(_ /* stack numbers */) => {
                stacks.into_iter()
                    .rev()
                    .for_each(|line| {
                        let mut it = line.chars().into_iter();
                        it.next(); // Skips first character '['
                        for (i, value) in it.step_by(4).enumerate() {
                            if value != ' ' {
                                crane.stacks[i].push(value)
                            }
                        }
                    });
                Ok(crane)
            }
            _ => Err(format!("Oops"))
        }
    }
}

trait CrateMover9000
{
    fn apply_instr(&mut self, instr: &CraneInstruction);
}

trait CrateMover9001
{
    fn apply_instr(&mut self, instr: &CraneInstruction);
}

impl CrateMover9000 for Crane {
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

impl CrateMover9001 for Crane {
    fn apply_instr(&mut self, instr: &CraneInstruction) {
        assert!(instr.from < self.stacks.len());
        assert!(instr.to < self.stacks.len());

        if instr.from != instr.to {
            let at = self.stacks[instr.from].len() - instr.num_crates;
            let mut x = self.stacks[instr.from].split_off(at);
            self.stacks[instr.to].append(&mut x);
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
    fn parse(str: &str) -> Result<CraneInstruction, Box<dyn std::error::Error>> {
        let split: Vec<&str> = str.split(' ').collect();
        match split[..] {
            [_, num, _, from, _, to] => {
                let from: usize = from.parse()?;
                let to: usize = to.parse()?;
                let instr = CraneInstruction {
                    num_crates: num.parse()?,
                    from: from - 1,
                    to: to - 1,
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
    let mut reader = utils::input_file_reader(files_dir)?;
    let mut lines = String::new();
    reader.read_to_string(&mut lines)?;

    let Some((header, data)) = lines.split_once("\n\n") else {
        panic!("Oops!");
    };

    let mut crane = Crane::parse(header)?;
    data.split("\n")
        .filter_map(|line| CraneInstruction::parse(line).ok())
        .for_each(|instr| CrateMover9000::apply_instr(&mut crane, &instr));

    let answer: String = crane.stacks.iter()
        .filter_map(|x| x.last().cloned())
        .collect();

    //println!("{:#?}", crane);
    println!("Part one top crates: {}", answer);
    Ok(())
}

fn part_two(files_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = utils::input_file_reader(files_dir)?;
    let mut lines = String::new();
    reader.read_to_string(&mut lines)?;

    let Some((header, data)) = lines.split_once("\n\n") else {
        panic!("Oops!");
    };

    let mut crane = Crane::parse(header)?;
    data.split("\n")
        .filter_map(|line| CraneInstruction::parse(line).ok())
        .for_each(|instr| CrateMover9001::apply_instr(&mut crane, &instr));

    let answer: String = crane.stacks.iter()
        .filter_map(|x| x.last().cloned())
        .collect();

    //println!("{:#?}", crane);
    println!("Part two top crates: {}", answer);
    Ok(())
}

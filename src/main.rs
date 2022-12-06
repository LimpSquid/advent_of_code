mod utils;

use lazy_static::lazy_static;
use std::env;
use clap::Parser;

macro_rules! count {
    () => (0usize);
    ($x:tt $($xs:tt)*) => (1usize + count!($($xs)*));
}

macro_rules! files_dir {
    ($module:ident) => {
        env::current_dir().unwrap()
            .display()
            .to_string()
        + &format!("/files/{}", String::from(stringify!($module)).to_lowercase())
    };
}

macro_rules! aoc_progs {
    ($(day($day:literal) = $module:ident),+) => {
        $(mod $module;)+
        lazy_static! {
            static ref PROGS: [(u8, fn(String) -> Result<(), Box<dyn std::error::Error>>, String); count!($($module)+)] = [
                $(($day, $module::solve, files_dir!($module)),)+
            ];
        }
    }
}

aoc_progs!
{
    day(1) = elves_calories,
    day(2) = rock_paper_scissors,
    day(3) = rucksack_reorganization,
    day(4) = camp_cleanup,
    day(5) = supply_stacks
}

fn run_prog(day: u8) {
    let prog = PROGS.iter()
        .find(|t| t.0 == day)
        .map(|t| Some((t.1, t.2.clone())));
    let Some(prog) = prog else {
        println!("Oops no result :-(...");
        return;
    };

    let (h, files_dir) = prog.unwrap();
    h(files_dir).unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   // Advent of code day
   #[arg(short, long, required = true)]
   day: u8,
}

fn main() {
    let args = Args::parse();
    run_prog(args.day);
}

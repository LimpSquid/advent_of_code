use std::fs::File;
use std::io::BufReader;

pub fn input_file_reader(filepath: String) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(filepath + "/input")?;
    Ok(BufReader::new(file))
}

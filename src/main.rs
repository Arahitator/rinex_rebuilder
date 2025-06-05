use rinex::prelude::*;
use std::{env, error, path::Path};

const REQUIRED_NUMBER_OF_ARGS: usize = 2;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(&args.len() > &REQUIRED_NUMBER_OF_ARGS, "input and output file path is required!");
    let input_file = &args[1];
    let output_file = &args[2];
    println!("Start to rebuild rinex {}", input_file);
    let rinex = Rinex::from_file(input_file)?;
    let path = Path::new(output_file);
    rinex.to_file(path)?;
    println!("Rinex rebuild into {} is done", output_file);
    Ok(())
}

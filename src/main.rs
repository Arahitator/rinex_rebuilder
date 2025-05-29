use rinex::prelude::*;
use std::{env, error, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Start to convert");
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let rinex = Rinex::from_file(input_file)?;
    let path = Path::new(output_file);
    rinex.to_file(path)?;
    println!("Convert done");
    Ok(())
}

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::PathBuf;

/// Clear rinex file from bad epoches or other events with non-0 epoch flag
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path of input file
    #[arg(short = 'i', long = "input", value_name = "PATH")]
    input_file: PathBuf,
    /// The path of output file
    #[arg(short = 'o', long = "output", value_name = "PATH")]
    output_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let input_file = File::open(cli.input_file).unwrap();
    let mut output_file = File::create(cli.output_file).unwrap();
    let reader = BufReader::new(input_file);
    let mut write = true;
    let mut is_header = true;
    let mut version: u8 = 0;

    for line in reader.lines() {
        let line_content = line.unwrap();
        if is_header {
            writeln!(output_file, "{}", line_content).unwrap();
            if line_content.contains("RINEX VERSION") {
                version = line_content[..11].trim().parse::<f32>().unwrap() as u8;
            }
            if line_content.contains("END OF HEADER") {
                is_header = false;
            }
        } else {
            if version == 3 {
                if line_content.chars().count() > 33 && line_content.starts_with(">") {
                    let epoch_flag = line_content[30..33].trim();
                    if epoch_flag == "0" {
                        write = true;
                    } else {
                        write = false;
                    }
                }
            } else if version == 2 {
                if line_content.chars().count() > 30 && !line_content[..2].trim().is_empty() {
                    let epoch_flag = line_content[27..30].trim();
                    if epoch_flag.len() == 1 {
                        if epoch_flag == "0" {
                            write = true;
                        } else {
                            write = false;
                        }
                    }
                }
            }
            if write {
                writeln!(output_file, "{}", line_content).unwrap();
            }
        }
    }
}

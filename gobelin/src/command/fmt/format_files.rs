use libgobelin::{format_gobelin_file, parse_gobelin_file, Config};
use std::fs::{write, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn format_files(
    config: &Result<Config, String>,
    files: &Vec<PathBuf>,
    verbose: bool,
) -> Result<(), String> {
    let config = config.clone()?;
    if files.is_empty() {
        return Err(String::from(
            "You should specify at least one file to the fmt command.",
        ));
    }
    for file in files.iter() {
        reformat(&config, file, verbose)?;
    }
    Ok(())
}

fn reformat(config: &Config, file: &PathBuf, verbose: bool) -> Result<(), String> {
    let original_content = read_as_string(file)?;
    let parsed = parse_gobelin_file(config, &original_content)?;
    let formatted = format_gobelin_file(config, &parsed)?;
    if formatted == original_content {
        if verbose {
            println!("{:?} is already formatted", file);
        }
    } else {
        write_file(file, &formatted)?;
        if verbose {
            println!("{:?} has been formatted", file);
        }
    }
    Ok(())
}

fn read_as_string(file: &PathBuf) -> Result<String, String> {
    let mut file = File::open(file).map_err(|e| format!("Cannot open file: {:?}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Cannot read file: {:?}", e))?;
    return Ok(contents);
}

fn write_file(file: &PathBuf, formatted: &String) -> Result<(), String> {
    write(file, formatted).map_err(|e| format!("Cannot write formatted file: {:?}", e))?;
    Ok(())
}

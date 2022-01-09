use libgobelin::{format_gobelin_file, parse_gobelin_file, Config};
use std::fs::{write, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn format_files(
    config: &Config,
    files: &[PathBuf],
    stdout: bool,
    verbose: bool,
) -> Result<(), String> {
    if files.is_empty() {
        return Err(String::from(
            "You should specify at least one file to the fmt command.",
        ));
    }
    let verbose = if stdout { false } else { verbose };
    for file in files.iter() {
        reformat(config, file, stdout, verbose, files.len() > 1)?;
    }
    Ok(())
}

fn reformat(
    config: &Config,
    file: &Path,
    stdout: bool,
    verbose: bool,
    multiple_files: bool,
) -> Result<(), String> {
    let original_content = read_as_string(file)?;
    let parsed = parse_gobelin_file(config, original_content.clone())?;
    let formatted = format_gobelin_file(config, &parsed)?;

    if stdout {
        print_reformat(file, &formatted, multiple_files);
    } else {
        write_in_file_reformat(file, &original_content, &formatted, verbose)?;
    }

    Ok(())
}

fn print_reformat(file: &Path, formatted: &String, multiple_files: bool) {
    if multiple_files {
        println!(
            ">>>>>>>>>>{}",
            file.canonicalize().unwrap().to_str().unwrap()
        );
    }
    println!("{}", formatted);
    if multiple_files {
        println!(
            "<<<<<<<<<<{}",
            file.canonicalize().unwrap().to_str().unwrap()
        );
    }
}

fn write_in_file_reformat(
    file: &Path,
    original_content: &String,
    formatted: &String,
    verbose: bool,
) -> Result<(), String> {
    if formatted == original_content {
        if verbose {
            println!("{:?} is already formatted", file);
        }
    } else {
        write_file(file, formatted.clone())?;
        if verbose {
            println!("{:?} has been formatted", file);
        }
    }
    Ok(())
}

fn read_as_string(file: &Path) -> Result<String, String> {
    let mut file = File::open(file).map_err(|e| format!("Cannot open file: {:?}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Cannot read file: {:?}", e))?;
    Ok(contents)
}

fn write_file(file: &Path, formatted: String) -> Result<(), String> {
    write(file, formatted).map_err(|e| format!("Cannot write formatted file: {:?}", e))?;
    Ok(())
}

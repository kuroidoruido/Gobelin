use chrono::Utc;
use libgobelin::{init_gobelin_directory, Locale};
use std::fs;
use std::path::PathBuf;

pub fn init_directory(
    root: &Option<PathBuf>,
    accounts: &Vec<String>,
    locale: Option<Locale>,
    verbose: bool,
) -> Result<(), String> {
    let root = root.clone().or(Some(PathBuf::from("."))).unwrap();
    if !root.exists() {
        fs::create_dir_all(root.clone())
            .map_err(|e| format!("Cannot create root directory {:?}", e))?;
        if verbose {
            println!("Root directory {:?} created", root);
        }
    }

    let (year, month) = todays_year_month();
    let files = init_gobelin_directory(accounts, locale, year, month)?;
    if verbose {
        println!(
            "Will create following files: {:?}",
            files.clone().into_keys().collect::<Vec<_>>()
        );
    }

    for (path, content) in files.iter() {
        let mut file_path = root.clone();
        file_path.push(path);
        if file_path.exists() {
            if verbose {
                println!("{:?} skipped (already exist)", file_path.clone());
            }
            continue;
        }

        let parent_directory = file_path.parent().unwrap();
        if !parent_directory.exists() {
            fs::create_dir_all(parent_directory).map_err(|e| {
                format!(
                    "Cannot create directory structure for {:?}: {:?}",
                    file_path.clone(),
                    e
                )
            })?;
        }
        fs::write(file_path.clone(), content)
            .map_err(|e| format!("Cannot create file {:?}: {:?}", file_path.clone(), e))?;
        if verbose {
            println!("{:?} created", file_path.clone());
        }
    }

    Ok(())
}

fn todays_year_month() -> (i32, u32) {
    let now = Utc::today();
    let year = now.format("%Y");
    let month = now.format("%m");

    let year = year.to_string().parse::<i32>().unwrap();
    let month = month.to_string().parse::<u32>().unwrap();
    (year, month)
}

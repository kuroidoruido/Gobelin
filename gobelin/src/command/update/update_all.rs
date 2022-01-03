use libgobelin::{update_all_files, Config};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn update_all(root: &Option<PathBuf>, config: &Config, verbose: bool) -> Result<(), String> {
    let root = root.clone().or_else(|| Some(PathBuf::from("."))).unwrap();
    let gobelin_files = read_all_gobelin_files(&root, verbose)?;
    let updated_files = update_all_files(config, &gobelin_files)?;
    write_all_gobelin_files(&gobelin_files, &updated_files, verbose)?;
    Ok(())
}

fn read_all_gobelin_files(root: &Path, verbose: bool) -> Result<BTreeMap<String, String>, String> {
    // expected structure is:
    // - one directory for each years at root
    // - in each year's directory, one file for each months

    if !root.exists() {
        return Err(format!("Root directory does not exists: {:?}", root));
    }

    let mut files: BTreeMap<String, String> = BTreeMap::new();
    for year_dir in root
        .read_dir()
        .map_err(|e| format!("Cannot read directory {:?}: {:?}", root, e))?
    {
        let year_dir =
            year_dir.map_err(|e| format!("Get an error when reading year dir: {:?}", e))?;
        let year_dir = year_dir.path();
        if !year_dir.is_dir() {
            continue;
        }
        for month_file in year_dir
            .read_dir()
            .map_err(|e| format!("Cannot read directory {:?}: {:?}", year_dir.clone(), e))?
        {
            let month_file =
                month_file.map_err(|e| format!("Get an error when reading month file: {:?}", e))?;
            let month_file_path = month_file.path();
            let month_file_path = month_file_path
                .to_str()
                .ok_or(format!("Cannot read month file extension {:?}", month_file))?;
            if month_file_path.ends_with(".gobelin") {
                let month_file_content = fs::read_to_string(month_file_path)
                    .map_err(|e| format!("Cannot read file {:?}: {:?}", month_file_path, e))?;
                files.insert(String::from(month_file_path), month_file_content);
            } else if verbose {
                println!("{:?} ignored", month_file);
            }
        }
    }

    Ok(files)
}

fn write_all_gobelin_files(
    actual_files: &BTreeMap<String, String>,
    files: &BTreeMap<String, String>,
    verbose: bool,
) -> Result<(), String> {
    for (file_path, file_content) in files.iter() {
        if actual_files.get(file_path).unwrap() == file_content {
            println!("{} already up to date", file_path);
            continue;
        }
        fs::write(file_path, file_content)
            .map_err(|e| format!("Cannot write file {:?}: {:?}", file_path.clone(), e))?;
        if verbose {
            println!("{} updated", file_path);
        }
    }
    Ok(())
}

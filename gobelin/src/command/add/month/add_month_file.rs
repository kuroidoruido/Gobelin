use libgobelin::{create_month_file, parse_gobelin_file, Config, GobelinFile};
use std::fs;
use std::path::PathBuf;

pub fn add_month_file(
    config: &Config,
    base_path: String,
    year: i32,
    month: u32,
    verbose: bool,
) -> Result<(), String> {
    let new_month_file = forge_year_month_file_name(base_path.clone(), year, month);
    if new_month_file.exists() {
        if verbose {
            println!(
                "Nothing to do. The file {:?} already exists.",
                new_month_file
            );
        }
        return Ok(());
    }

    let previous_month = get_previous_month(config, base_path, year, month);
    let new_month_content = match previous_month {
        Ok(previous_month) => create_month_file(config, &previous_month, year, month),
        Err(e) => Err(e),
    }?;

    let new_month_parent = new_month_file.parent().unwrap();
    if !new_month_parent.exists() {
        fs::create_dir_all(new_month_parent).map_err(|e| {
            format!(
                "Cannot create directory structure for {:?}: {:?}",
                new_month_file.clone(),
                e
            )
        })?;
    }
    fs::write(new_month_file.clone(), new_month_content)
        .map_err(|e| format!("Cannot create file {:?}: {:?}", new_month_file.clone(), e))?;

    Ok(())
}

fn forge_year_month_file_name(base_path: String, year: i32, month: u32) -> PathBuf {
    [base_path, format!("{}/{}.gobelin", year, month)]
        .iter()
        .collect::<PathBuf>()
}

fn get_previous_month(
    config: &Config,
    base_path: String,
    year: i32,
    month: u32,
) -> Result<GobelinFile, String> {
    let previous_month_month = if month == 1 { 12 } else { month - 1 };
    let previous_month_year = if previous_month_month > month {
        year - 1
    } else {
        year
    };
    let previous_month_file =
        forge_year_month_file_name(base_path, previous_month_year, previous_month_month);

    if !previous_month_file.exists() {
        return Err(format!(
            "Cannot find previous month (expected here: {:?}), so cannot create next month.",
            previous_month_file
        ));
    }
    let previous_month = fs::read_to_string(previous_month_file.clone()).map_err(|e| {
        format!(
            "Cannot read previous month {:?}: {:?}",
            previous_month_file.clone(),
            e
        )
    })?;
    parse_gobelin_file(config, previous_month)
}

use libgobelin::{AccountConfig, Config, GeneralConfig, Locale};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Debug)]
pub struct ConfigDef {
    #[serde(with = "GeneralConfigDef")]
    pub general: GeneralConfig,
    pub accounts: BTreeMap<String, AccountConfigDef>,
}

impl ConfigDef {
    fn to_config(&self) -> Config {
        Config {
            general: self.general.clone(),
            accounts: self
                .accounts
                .iter()
                .map(|(_, a)| a)
                .map(AccountConfigDef::to_account_config)
                .collect(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(remote = "GeneralConfig")]
pub struct GeneralConfigDef {
    #[serde(with = "LocaleDef")]
    pub locale: Locale,
}

// Serde calls this the definition of the remote type. It is just a copy of the
// remote data structure. The `remote` attribute gives the path to the actual
// type we intend to derive code for.
#[derive(Deserialize)]
#[serde(remote = "Locale")]
enum LocaleDef {
    FR,
    EN,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AccountConfigDef {
    pub name: String,
}

impl AccountConfigDef {
    fn to_account_config(&self) -> AccountConfig {
        AccountConfig {
            name: self.name.clone(),
        }
    }
}

pub fn parse_config(base_path: &Option<PathBuf>) -> Result<(Config, String), String> {
    let base_path: String = match base_path {
        Some(path) => String::from(path.to_str().unwrap()),
        None => String::from("."),
    };
    let config_file_path: PathBuf = [base_path.as_str(), "gobelin.toml"].iter().collect();
    let config_file_path: &str = config_file_path.to_str().unwrap();
    let config_file = read_to_string(config_file_path).map_err(|e| {
        format!(
            "Cannot read config file with path {:?}: {:?}",
            config_file_path, e
        )
    })?;
    let config: ConfigDef = toml::from_str(&config_file).map_err(|e| {
        format!(
            "Cannot deserialize config file with path {:?}: {:?}",
            config_file_path, e
        )
    })?;
    Ok((config.to_config(), base_path))
}

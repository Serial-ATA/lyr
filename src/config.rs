use crate::error::{Error, Result};
use crate::fetcher::DEFAULT_FETCHERS;

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub(crate) flags: String,
    pub(crate) fetchers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            flags: String::new(),
            fetchers: DEFAULT_FETCHERS.clone(),
        }
    }
}

impl Config {
    pub fn read() -> Result<Self> {
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir,
            None => {
                log::warn!("Failed to find config dir, using default config. Consider setting `XDG_CONFIG_HOME`.");
                return Ok(Self::default());
            }
        };

        let config_dir = config_dir.join("lyr");
        if !config_dir.exists() {
            fs::create_dir(&config_dir)?;
        }

        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            let ret = Config::default();
            fs::write(config_file.as_path(), toml::to_string_pretty(&ret).unwrap())?;
            return Ok(ret);
        }

        let mut conf: Config = toml::from_str(&fs::read_to_string(config_file)?)?;
        conf.fetchers = conf
            .fetchers
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>();

        let bad_keys = conf
            .fetchers
            .iter()
            .filter(|f| !DEFAULT_FETCHERS.contains(f))
            .collect::<Vec<_>>();
        if !bad_keys.is_empty() {
            return Err(Error::BadFetcher(format!(
                "{:?}, Valid keys are: {:?}",
                bad_keys, DEFAULT_FETCHERS
            )));
        }

        Ok(conf)
    }
}

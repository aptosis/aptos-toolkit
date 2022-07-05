//! Aptos CLI configuration utilities extracted from the official Aptos CLI.
//!
//! This package exists as a lightweight alternative to the `aptos` crate, reducing
//! the number of dependencies needed to interface with the Aptos CLI configuration.
use aptos_crypto::ed25519::{Ed25519PrivateKey, Ed25519PublicKey};
use aptos_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use utils::{from_yaml, open_file, read_from_file, write_to_user_only_file};

pub mod error;
pub mod utils;

pub use error::*;

/// Config saved to `.aptos/config.yaml`
#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    /// Map of profile configs
    pub profiles: Option<HashMap<String, ProfileConfig>>,
}

const CONFIG_FILE: &str = "config.yaml";
const LEGACY_CONFIG_FILE: &str = "config.yml";
const CONFIG_FOLDER: &str = ".aptos";

/// An individual profile
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProfileConfig {
    /// Private key for commands.
    pub private_key: Option<Ed25519PrivateKey>,
    /// Public key for commands
    pub public_key: Option<Ed25519PublicKey>,
    /// Account for commands
    pub account: Option<AccountAddress>,
    /// URL for the Aptos rest endpoint
    pub rest_url: Option<String>,
    /// URL for the Faucet endpoint (if applicable)
    pub faucet_url: Option<String>,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            profiles: Some(HashMap::new()),
        }
    }
}

impl CliConfig {
    /// Checks if the config exists in the current working directory
    pub fn config_exists() -> bool {
        if let Ok(folder) = Self::aptos_folder() {
            let config_file = folder.join(CONFIG_FILE);
            let old_config_file = folder.join(LEGACY_CONFIG_FILE);
            config_file.exists() || old_config_file.exists()
        } else {
            false
        }
    }

    /// Loads the config from the current working directory
    pub fn load() -> CliTypedResult<Self> {
        let folder = Self::aptos_folder()?;

        let config_file = folder.join(CONFIG_FILE);
        let old_config_file = folder.join(LEGACY_CONFIG_FILE);
        if config_file.exists() {
            from_yaml(
                &String::from_utf8(read_from_file(config_file.as_path())?)
                    .map_err(CliError::from)?,
            )
        } else if old_config_file.exists() {
            from_yaml(
                &String::from_utf8(read_from_file(old_config_file.as_path())?)
                    .map_err(CliError::from)?,
            )
        } else {
            Err(CliError::ConfigNotFoundError(format!(
                "{}",
                config_file.display()
            )))
        }
    }

    pub fn load_profile(profile: &str) -> CliTypedResult<Option<ProfileConfig>> {
        let mut config = Self::load()?;
        Ok(config.remove_profile(profile))
    }

    pub fn remove_profile(&mut self, profile: &str) -> Option<ProfileConfig> {
        if let Some(ref mut profiles) = self.profiles {
            profiles.remove(&profile.to_string())
        } else {
            None
        }
    }

    /// Saves the config to ./.aptos/config.yaml
    pub fn save(&self) -> CliTypedResult<()> {
        let aptos_folder = Self::aptos_folder()?;

        // Create if it doesn't exist
        if !aptos_folder.exists() {
            std::fs::create_dir(&aptos_folder).map_err(|err| {
                CliError::CommandArgumentError(format!(
                    "Unable to create {} directory {}",
                    aptos_folder.display(),
                    err
                ))
            })?;
        }

        // Save over previous config file
        let config_file = aptos_folder.join(CONFIG_FILE);
        let config_bytes = self.try_to_string()?;
        write_to_user_only_file(&config_file, CONFIG_FILE, config_bytes.as_bytes())?;

        // As a cleanup, delete the old if it exists
        let legacy_config_file = aptos_folder.join(LEGACY_CONFIG_FILE);
        if legacy_config_file.exists() {
            eprintln!("Removing legacy config file {}", LEGACY_CONFIG_FILE);
            let _ = std::fs::remove_file(legacy_config_file);
        }
        Ok(())
    }

    /// Finds the current directory's .aptos folder
    fn aptos_folder() -> CliTypedResult<PathBuf> {
        std::env::current_dir()
            .map_err(|err| {
                CliError::UnexpectedError(format!("Unable to get current directory {}", err))
            })
            .map(|dir| dir.join(CONFIG_FOLDER))
    }

    // additions by Aptosis

    /// Loads the config from the specified path.
    pub fn load_from_path(path: &Path) -> CliTypedResult<CliConfig> {
        Ok(serde_yaml::from_reader(open_file(path)?)?)
    }

    /// Loads the config from the package root.
    pub fn load_from_root(root: &Path) -> CliTypedResult<CliConfig> {
        Self::load_from_path(&root.join(CONFIG_FOLDER).join(CONFIG_FILE))
    }

    /// Serializes a [CliConfig] to a [String].
    pub fn try_to_string(&self) -> CliTypedResult<String> {
        serde_yaml::to_string(&self)
            .map_err(|err| CliError::UnexpectedError(format!("Failed to serialize config {}", err)))
    }
}

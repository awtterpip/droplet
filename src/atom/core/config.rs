use std::path::Path;

use super::error::Result;

use serde::Deserialize;

/// The main runtime configuration for `droplet`.
#[derive(Deserialize)]
pub struct Config {
    /// The inner [`Service`] configuration.
    service: Service,

    /// The optional inner [`Dns`] configuration.
    dns: Option<Dns>,

    /// The optional inner [`Sync`] configuration.
    sync: Option<Sync>,
}

impl Config {
    /// Attempts to parse a [Config] from the file at the path
    pub fn try_get(path: impl AsRef<Path>) -> Result<Self> {
        let string = std::fs::read_to_string(path)?;
        Self::from_str(&string)
    }

    /// Attempts to parse a [Config] from a string.
    ///
    /// Expects toml format
    pub fn from_str(string: &str) -> Result<Self> {
        Ok(toml::from_str(string)?)
    }
}

/// Getter functions for [`Config`].
impl Config {
    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn dns(&self) -> Option<&Dns> {
        self.dns.as_ref()
    }

    pub fn sync(&self) -> Option<&Sync> {
        self.sync.as_ref()
    }
}

/// The service configuration for `droplet`.
#[derive(Deserialize)]
pub struct Service {
    /// Path to the executable binary/script.
    exec: String,

    /// Arguments for the executable.
    #[serde(default)]
    args: Vec<String>,

    /// Whether to print output to the terminal (`false`) or log to a `service.log` file (`true`).
    #[serde(default)]
    log: bool,
}

impl Service {
    /// Run the service.
    ///
    /// This function does nothing, pending implementation.
    pub fn run(&self) -> Result<()> {
        // todo!()
        Ok(())
    }
}

/// Getter functions for [`Service`].
impl Service {
    pub fn exec(&self) -> &String {
        &self.exec
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn log(&self) -> bool {
        self.log
    }
}

/// The DNS configuration for `droplet`.
#[derive(Deserialize)]
pub struct Dns {
    /// [FreeDNS](https://freedns.afraid.org) Dynamic DNS code, used to update your domain's IP address.
    code: String,
}

impl Dns {
    /// Update your domain's destination IP address.
    ///
    /// This function does nothing, pending implementation.
    pub fn update_ip(&self) {
        // todo!()
    }
}

/// Getter functions for [`Dns`].
impl Dns {
    pub fn code(&self) -> &String {
        &self.code
    }
}

fn path_default() -> String {
    ".".into()
}

fn targets_default() -> Vec<String> {
    vec![".".into()]
}

/// The DNS configuration for `droplet`.
#[derive(Deserialize)]
pub struct Sync {
    /// URL to the git repository `droplet` will use.
    ///
    /// (ex. `https://github.com/username/repository`)
    origin: String,

    /// Relative path (on disk) to the git repository.
    #[serde(default = "path_default")]
    path: String,

    /// List of relative paths to files that should be kept in sync. Specifying a directory will synchronize all files within.
    #[serde(default = "targets_default")]
    targets: Vec<String>,
}

impl Sync {
    /// `git pull` changes from remote repository.
    ///
    /// This function does nothing, pending implementation.
    pub fn pull_changes(&self) -> Result<()> {
        // todo!()
        Ok(())
    }

    /// `git add` files to be synchronized.
    ///
    /// This function does nothing, pending implementation.
    pub fn add_changes(&self) -> Result<()> {
        // todo!()
        Ok(())
    }

    /// `git push` changes to remote repository.
    ///
    /// This function does nothing, pending implementation.
    pub fn push_changes(&self) -> Result<()> {
        // todo!()
        Ok(())
    }
}

/// Getter functions for [`Sync`].
impl Sync {
    pub fn origin(&self) -> &String {
        &self.origin
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn targets(&self) -> &Vec<String> {
        &self.targets
    }
}

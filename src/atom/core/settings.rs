use crate::atom::nucleus;

/// Runtime settings for `droplet`.
pub struct Settings {
    /// Relative path to the `droplet.toml` config file.
    config_path: String,

    /// Whether or not to update the domain to the current IP address.
    update_dns: bool,

    /// Whether or not to run `git pull` before running the service.
    sync_pull: bool,

    /// Whether or not to run `git push` after the service exits.
    sync_push: bool,
}

impl Settings {
    /// Create a [`Settings`] from the [`nucleus::args::Args`] created by [`nucleus::args::parse`].
    pub fn get() -> Settings {
        let args = nucleus::args::parse();

        let config_path = args.config;
        let update_dns = !args.no_dns;
        let sync_pull = !(args.no_pull || args.no_sync);
        let sync_push = !(args.no_push || args.no_sync);

        Settings {
            config_path,
            update_dns,
            sync_pull,
            sync_push,
        }
    }
}

/// Getter functions for [`Settings`].
impl Settings {
    pub fn config_path(&self) -> &String {
        &self.config_path
    }

    pub fn update_dns(&self) -> bool {
        self.update_dns
    }

    pub fn sync_pull(&self) -> bool {
        self.sync_pull
    }

    pub fn sync_push(&self) -> bool {
        self.sync_push
    }
}

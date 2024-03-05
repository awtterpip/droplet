/// Collection of exports for the [`crate::valence`] layer.
pub mod prelude {
    pub use super::config::Config;
    pub use super::settings::Settings;

    pub use super::config;

    pub use anyhow::Result;
}

/// Runtime settings for `droplet`, generally set at the commandline.
pub mod settings {
    use crate::nucleus;

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
}

/// Runtime configuration for `droplet`, generally derived from `droplet.toml`.
pub mod config {
    use super::error::Error;
    use super::settings::Settings;

    /// The main runtime configuration for `droplet`.
    pub struct Config {
        /// The inner [`Service`] configuration.
        service: Service,

        /// The optional inner [`Dns`] configuration.
        dns: Option<Dns>,

        /// The optional inner [`Sync`] configuration.
        sync: Option<Sync>,
    }

    impl Config {
        /// Create a [`Config`].
        ///
        /// This function returns a temporary default, pending implementation.
        pub fn get(settings: &Settings) -> Config {
            // todo!()
            Config::test_value()
        }

        /// Create a dummy [`Config`] for testing purposes.
        fn test_value() -> Config {
            let service = Service::test_value();
            let dns = Some(Dns::test_value());
            let sync = Some(Sync::test_value());

            Config { service, dns, sync }
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
    pub struct Service {
        /// Path to the executable binary/script.
        exec: String,

        /// Arguments for the executable.
        args: Vec<String>,

        /// Whether to print output to the terminal (`false`) or log to a `service.log` file (`true`).
        log: bool,
    }

    impl Service {
        /// Run the service.
        ///
        /// This function does nothing, pending implementation.
        pub fn run(&self) -> Result<(), Error> {
            // todo!()
            Ok(())
        }

        /// Create a default [`Service`] for testing purposes.
        fn test_value() -> Service {
            let exec = "/bin/sh".to_string();
            let args = vec!["-c".to_string(), "echo test".to_string()];
            let log = true;

            Service { exec, args, log }
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

        /// Create a default [`Dns`] for testing purposes.
        fn test_value() -> Dns {
            let code = "eWBRKL3JuN4W6KDj2vLkt94o".to_string();

            Dns { code }
        }
    }

    /// Getter functions for [`Dns`].
    impl Dns {
        pub fn code(&self) -> &String {
            &self.code
        }
    }

    /// The DNS configuration for `droplet`.
    pub struct Sync {
        /// URL to the git repository `droplet` will use.
        ///
        /// (ex. `https://github.com/username/repository`)
        origin: String,

        /// Relative path (on disk) to the git repository.
        path: String,

        /// List of relative paths to files that should be kept in sync. Specifying a directory will synchronize all files within.
        targets: Vec<String>,
    }

    impl Sync {
        /// `git pull` changes from remote repository.
        ///
        /// This function does nothing, pending implementation.
        pub fn pull_changes(&self) -> Result<(), Error> {
            // todo!()
            Ok(())
        }

        /// `git add` files to be synchronized.
        ///
        /// This function does nothing, pending implementation.
        pub fn add_changes(&self) -> Result<(), Error> {
            // todo!()
            Ok(())
        }

        /// `git push` changes to remote repository.
        ///
        /// This function does nothing, pending implementation.
        pub fn push_changes(&self) -> Result<(), Error> {
            // todo!()
            Ok(())
        }

        /// Create a default [`Sync`] for testing purposes.
        fn test_value() -> Sync {
            let origin = "https://github.com/SaphiraKai/droplet-sync-test".to_string();
            let path = "../droplet-sync-test/".to_string();
            let targets = vec!["update/".to_string()];

            Sync {
                origin,
                path,
                targets,
            }
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
}

/// TODO: Document [`crate::core::error`].
pub mod error {
    use std::{error::Error as ErrorTrait, fmt};

    #[derive(Debug)]
    pub enum Error {
        SyncError(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let msg = match self {
                Error::SyncError(s) => s,
            };

            write!(f, "{}", msg)
        }
    }

    impl ErrorTrait for Error {
        fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
            None
        }

        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }

        fn cause(&self) -> Option<&dyn ErrorTrait> {
            self.source()
        }
    }
}

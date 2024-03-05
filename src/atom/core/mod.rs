/// Collection of exports for the [`crate::atom::valence`] layer.
pub mod prelude {
    pub use super::config::Config;
    pub use super::settings::Settings;

    pub use super::config;

    pub use anyhow::Result;
}

/// Runtime settings for `droplet`, generally set at the commandline.
pub mod settings;

/// Runtime configuration for `droplet`, generally derived from `droplet.toml`.
pub mod config;

/// todo! Document [`crate::atom::core::error`].
pub mod error;

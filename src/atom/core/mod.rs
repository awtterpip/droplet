/// Collection of exports for the [`crate::valence`] layer.
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

/// TODO: Document [`crate::core::error`].
pub mod error;
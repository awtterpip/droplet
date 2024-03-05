use crate::core::prelude::*;

/// "Real" entrypoint into the program.
///
/// This function is called by [`crate::main`].
pub(crate) fn main() -> Result<()> {
    let settings = Settings::get();
    let config = Config::get(&settings);

    if settings.update_dns() {
        if let Some(dns) = config.dns() {
            dns.update_ip();
        }
    }

    if settings.sync_pull() {
        if let Some(sync) = config.sync() {
            sync.pull_changes()?;
        }
    }

    let service_result = config.service().run();

    if let Some(sync) = config.sync() {
        sync.add_changes()?;

        if settings.sync_push() {
            sync.push_changes()?;
        }
    }

    Ok(service_result?)
}
use std::{env::set_current_dir, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

mod util;

#[derive(Parser)]
struct Args {
    /// Path to config file
    #[arg(default_value_t = String::from("./droplet.toml"))]
    config: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = PathBuf::from(args.config)
        .canonicalize()
        .context("couldn't canonicalize config path")?;

    let config_parent_dir = config_path
        .parent()
        .context("couldn't determine parent directory")?;

    set_current_dir(config_parent_dir).context("failed changing directory")?;
    println!("changed directory to {}", config_parent_dir.display());

    let config = util::get_config(&config_path).context(format!(
        "failed loading config file {}",
        &config_path.display()
    ))?;
    println!("configuration loaded");

    let service = util::start_service(&config).context("failed starting service")?;
    println!("service started");

    let response = util::update_dns(&config);

    match response {
        Ok(r) => {
            for line in r.lines() {
                println!("dns: {line}");
            }
        }
        Err(e) => {
            eprintln!("\nwarning: failed to update DNS record");
            eprintln!("your service will only be accessible directly via your public IP address");
            eprintln!("\nsource of error:\n{e}");
        }
    }

    println!("\nall tasks complete, waiting for service to exit...");

    let output = service
        .wait_with_output()
        .context("failed waiting for service process")?;

    let exit_code = output.status.code();

    println!(
        "service exited with code {}",
        exit_code
            .map(|c| c.to_string())
            .unwrap_or("<none>".to_string())
    );

    Ok(())
}

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Path to config file
    #[arg(default_value_t = String::from("./droplet.toml"))]
    pub config: String,

    /// Don't update DNS
    #[arg(long)]
    pub no_dns: bool,

    /// Don't pull changes from remote
    #[arg(long)]
    pub no_pull: bool,

    /// Don't push changes to remote
    #[arg(long)]
    pub no_push: bool,

    /// Alias for --no-pull --no-push
    #[arg(long)]
    pub no_sync: bool,
}

pub fn parse() -> Args {
    Args::parse()
}

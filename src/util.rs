use std::{
    fs::{read_to_string, remove_file, File},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

use anyhow::{bail, Ok, Result};
use toml::Table;

static DEFAULT_CONFIG: &str = r#"
    service.args = []
"#;

macro_rules! missing {
    ($key:expr) => {
        bail!("no value was provided for '{}'", $key);
    };
}
macro_rules! invalid {
    ($key:expr, $type:expr) => {
        bail!("value for '{}' must be '{}'", $key, $type);
    };
}

pub(crate) fn get_config<P>(path: P) -> Result<Table>
where
    P: AsRef<Path>,
{
    let mut config: Table = DEFAULT_CONFIG.parse()?;

    let toml: Table = read_to_string(path)?.parse()?;

    for (key, value) in toml {
        if config.contains_key(&key) {
            config[&key] = value;
            continue;
        }

        config.insert(key, value);
    }

    Ok(config)
}

pub(crate) fn get_dns_code(config: &Table) -> Result<String> {
    let key = "dns_code";
    let value_type = "string";

    let Some(value) = config.get(key) else {
        missing!(key);
    };

    let Some(code) = value.as_str() else {
        invalid!(key, value_type);
    };

    Ok(code.to_string())
}

pub(crate) fn update_dns(config: &Table) -> Result<String> {
    let code = get_dns_code(config)?;

    let url = format!("http://sync.afraid.org/u/{code}/");
    let mut response = reqwest::blocking::get(url)?.text()?;

    if response.ends_with('\n') {
        response.pop();
    }

    Ok(response)
}

pub(crate) fn get_service(config: &Table) -> Result<&Table> {
    let key = "service";
    let value_type = "table";

    let exec_key = "exec";
    let exec_value_type = "string";

    let args_key = "args";
    let args_value_type = "array of strings";

    let Some(value) = config.get(key) else {
        missing!(key);
    };

    let Some(table) = value.as_table() else {
        invalid!(key, value_type);
    };

    let Some(exec_value) = table.get(exec_key) else {
        missing!(format!("{key}.{exec_key}"));
    };

    let Some(exec_str) = exec_value.as_str() else {
        invalid!(key, exec_value_type);
    };

    if !PathBuf::from(exec_str).exists() {
        bail!("service file '{exec_str}' does not exist");
    }

    let Some(args_value) = table.get(args_key) else {
        missing!(format!("{key}.{args_key}"));
    };

    let Some(args_array) = args_value.as_array() else {
        invalid!(format!("{key}.{args_key}"), args_value_type);
    };

    if args_array.iter().any(|i| i.as_str().is_none()) {
        invalid!(format!("{key}.{args_key}"), args_value_type);
    };

    Ok(table)
}

pub(crate) fn start_service(config: &Table) -> Result<Child> {
    let service = get_service(config)?;

    let log_path = PathBuf::from("service.log");
    if log_path.exists() {
        remove_file(log_path)?;
    }

    let stdout_log = File::create("service.log").unwrap();
    let stderr_log = stdout_log.try_clone()?;

    let exec = service["exec"].as_str().unwrap();
    let args: Vec<&str> = service["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap())
        .collect();

    let child = Command::new(exec)
        .args(args)
        .stdout(Stdio::from(stdout_log))
        .stderr(Stdio::from(stderr_log))
        .spawn()?;

    Ok(child)
}

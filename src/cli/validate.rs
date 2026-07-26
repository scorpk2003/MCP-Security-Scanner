use std::{fs, path::{Path, PathBuf}, str::FromStr};

use crate::TargetType;


pub fn validate_path(path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if p.exists() {
        Ok(p.to_path_buf())
    } else {
        Err(format!("Path `{path}` does not exist!!!"))
    }
}

pub fn validate_dir(path: &str) -> Result<PathBuf, String> {
    let path_validate = validate_path(path)?;
    if path_validate.is_dir() {
        Ok(path_validate)
    } else {
        Err(format!("Path `{path}` is not a directory!!!"))
    }
}

pub fn validate_conf(path: &str) -> Result<PathBuf, String> {
    let path_validate = validate_path(path)?;
    if path_validate.is_file() {
        if let Err(_) = fs::read_to_string(path_validate.clone()) {
            return Err(format!("Can't read path `{path}`!!!"));
        }
        Ok(path_validate)
    } else {
        Err(format!("Path `{path}` is not a file!!!"))
    }
}

pub fn validate_timeout(timeout: &str) -> Result<u64, String> {
    let tms = timeout.parse::<u64>().expect("Time out must be nature number");
    if tms.le(&0u64) {
        return Err(format!("Time out is negative!!!"))
    }
    Ok(tms)
}

impl FromStr for TargetType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s.to_lowercase().as_str() {
            "json" => Ok(TargetType::JSON),
            "markdown" | "md" => Ok(TargetType::MARKDOWN),
            "console" => Ok(TargetType::CONSOLE),
            _ => Err(format!("Unknown Command `{s}`"))
        }
    }
}
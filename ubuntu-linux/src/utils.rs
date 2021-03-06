use crate::environment::Environment;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;
use anyhow::{Result, Context};

pub fn write_to_file(filename: &str, text: &str) -> Result<()>{
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename).with_context(|| format!("Failed to open file {}", filename))?;
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
        Ok(())
    }

pub fn apt_update() -> Result<()> {
    let mut c = Command::new("apt-get")
        .arg("update")
        .spawn().with_context(|| format!("apt update failed"))?;
    c.wait().with_context(|| format!("apt update failed"))?;
    Ok(())
}

pub fn apt_install<'a>(mut packages: Vec<&'a str>, options: Option<Vec<&'a str>>) -> Result<()>{
    let mut cmd_line = vec!["install"];
    cmd_line.append(&mut packages);
    cmd_line.append(&mut options.unwrap_or(vec![""]));
    let mut c = Command::new("apt-get")
        .args(cmd_line)
        .spawn().with_context(|| format!("Install failed with error"))?;
    c.wait().with_context(|| format!("Install failed with error"))?;
    Ok(())
}

pub fn git_clone(repo_address: &str, destination: &str) -> Result<()> {
    let mut c = Command::new("git")
        .args(&["clone", "--depth=1", repo_address, destination])
        .spawn().with_context(|| format!("Git clone failed with error"))?;
    c.wait().with_context(|| format!("Git clone failed with error"))?;
    Ok(())
}

pub fn pip_install<'a>(mut packages: Vec<&'a str>, options: Option<Vec<&'a str>>) -> Result<()> {
    let env_vars: Environment = Environment::load().with_context(|| format!("Failed getting environment variable"))?;
    let mut cmd_line = vec!["-m", "pip", "install"];
    cmd_line.append(&mut options.unwrap_or(vec![""]));
    cmd_line.append(&mut packages);
    let mut c = Command::new(&format!("{}/.pyenv/versions/{}/bin/python3", env_vars.home_var, env_vars.python_version))
        .args(cmd_line)
        .spawn().with_context(|| format!("pip install failed"))?;
    c.wait().with_context(|| format!("pip install failed"))?;
    Ok(())
}

pub fn pipx_install (mut packages: Vec<&str>) -> Result<()> {
    let env_vars: Environment = Environment::load().with_context(|| format!("Failed getting environment variable"))?;
    let mut c = Command::new(&format!("{}/.local/bin/pipx", env_vars.home_var))
        .arg("install")
        .args(&mut packages)
        .spawn().with_context(|| format!("pipx install failed"))?;
    c.wait().with_context(|| format!("pipx install failed"))?;
    Ok(())
}

mod utils;
mod environment;

use crate::utils::{git_clone, apt_install, apt_update,
                   write_to_file, pip_install, pipx_install};
use crate::environment::Environment;
use std::process::{Command, Stdio};
use std::env;
use anyhow::{Result, Context};


fn main() -> Result<()> {
    apt_update()?;
    apt_install(vec!["lsb-release"], Some(vec!["-y"]))?;
    let env_vars: Environment = Environment::load()?;
    apt_install(vec!["zsh", "curl", "wget", "git"], Some(vec!["-y"]))?;

    let mut c = Command::new("wget")
        .arg("https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh")
        .spawn().with_context(|| format!("Failed running wget"))?;
    c.wait().with_context(|| format!("Failed running wget"))?;

    git_clone("https://github.com/junegunn/fzf.git", &format!("{}/.fzf", env_vars.home_var))?;

    let mut c = Command::new(&format!("{}/.fzf/install", env_vars.home_var))
        .arg("--all")
        .spawn().with_context(|| format!("Failed to install fzf"))?;
    c.wait().with_context(|| format!("Failed running wget"))?;

    apt_update()?;
    env::set_var("DEBIAN_FRONTEND", "noninteractive");
    apt_install(vec!["tzdata"], Some(vec!["-y", "--no-install-recommends"]))?;
    apt_install(vec!["make", "build-essential", "libssl-dev", "zlib1g-dev", "libbz2-dev",
                     "libreadline-dev", "libsqlite3-dev", "llvm",
                     "libncurses5-dev", "xz-utils", "tk-dev",
                     "libxml2-dev", "libxmlsec1-dev", "libffi-dev", "liblzma-dev"], Some(vec!["-y", "--no-install-recommends"]))?;
    git_clone("https://github.com/pyenv/pyenv.git", &format!("{}/.pyenv", env_vars.home_var))?;
    write_to_file(&format!("{}/.bashrc", env_vars.home_var), &format!("export PYENV_ROOT=\"{}/.pyenv\"", env_vars.home_var))?;
    write_to_file(&format!("{}/.bashrc", env_vars.home_var), &format!("export PATH=\"{}/bin:{}\"",format!("{}/.pyenv", env_vars.home_var), env_vars.path_var))?;
    write_to_file(&format!("{}/.bashrc", env_vars.home_var), "if command -v pyenv 1>/dev/null 2>&1; then\n  eval \"$(pyenv init -)\"\nfi")?;

    let mut c = Command::new(&format!("{}/.pyenv/bin/pyenv", env_vars.home_var))
        .args(&["install", &env_vars.python_version])
        .spawn().with_context(|| format!("Failed to install python {} with pyenv", {&env_vars.python_version}))?;
    c.wait().with_context(|| format!("Failed to install python {} with pyenv", {&env_vars.python_version}))?;

    pip_install(vec!["pip"], Some(vec!["-U"]))?;
    pip_install(vec!["pipx"], Some(vec!["--user"]))?;

    let mut c = Command::new(&format!("{}/.pyenv/versions/{}/bin/python3", env_vars.home_var, env_vars.python_version))
        .args(&["-m", "pipx", "ensurepath"])
        .spawn().with_context(|| format!("Failed running pipx ensurepath"))?;
    c.wait().with_context(|| format!("Failed running pipx ensurepath"))?;

    pipx_install(vec!["poetry"])?;
    pipx_install(vec!["pipenv"])?;
    apt_update()?;
    apt_install(vec!["apt-transport-https", "ca-certificates", "curl", "gnupg"], Some(vec!["-y"]))?;

    let mut c = Command::new("curl")
        .args(&["-fsSL", "https://download.docker.com/linux/ubuntu/gpg"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(prior_command) = c.stdout.take() { // if it's not None
        let mut c2 = Command::new("gpg")
            .args(&["--dearmor", "-o", "/usr/share/keyrings/docker-archive-keyring.gpg"])
            .stdin(prior_command)
            .spawn().with_context(|| format!("GPG keyring error"))?;
        c2.wait().with_context(|| format!("GPG keyring error"))?;
    }
    apt_update()?;
    write_to_file("/etc/apt/sources.list.d/docker.list",
                  &format!("deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu {:?} stable", env_vars.ubuntu_distribution))?;
    apt_update()?;
    apt_install(vec!["docker-ce", "docker-ce-cli", "containerd.io"], Some(vec!["-y"]))?;
    apt_install(vec!["vim"], Some(vec!["-y"]))?;
    Ok(())
}
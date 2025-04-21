use anyhow::{Result, anyhow};
use std::path::Path;
use which::which;

pub fn vimdiff_command(file1: &Path, file2: &Path) -> Result<(String, Vec<String>)> {
    check_command_exists("vim")?;
    Ok((
        "vim".to_string(),
        vec![
            "-d".to_string(),
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
    ))
}

pub fn meld_command(file1: &Path, file2: &Path) -> Result<(String, Vec<String>)> {
    check_command_exists("meld")?;
    Ok((
        "meld".to_string(),
        vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
    ))
}

pub fn kdiff3_command(file1: &Path, file2: &Path) -> Result<(String, Vec<String>)> {
    check_command_exists("kdiff3")?;
    Ok((
        "kdiff3".to_string(),
        vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
    ))
}

pub fn vscode_command(file1: &Path, file2: &Path) -> Result<(String, Vec<String>)> {
    // Try to find either 'code' or 'codium' command
    let cmd = if which("code").is_ok() {
        "code"
    } else if which("codium").is_ok() {
        "codium"
    } else {
        return Err(anyhow!("VS Code is not installed or not in PATH"));
    };

    Ok((
        cmd.to_string(),
        vec![
            "--diff".to_string(),
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
    ))
}

fn check_command_exists(cmd: &str) -> Result<()> {
    which(cmd).map_err(|_| anyhow!("Command '{}' not found in PATH", cmd))?;
    Ok(())
}

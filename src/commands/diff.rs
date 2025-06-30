use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, anyhow};
use colored::*;

use crate::models::DotPath;
use crate::utils::diff_tools;

enum DiffStatus {
    Added,
    Modified,
    Removed,
}

pub fn show_diff(dotfile_path: DotPath, tool: Option<String>, word_diff: bool) -> Result<()> {
    if !dotfile_path.abs_path.exists() {
        println!(
            "{} Local path does not exist: {}",
            "✗".red(),
            dotfile_path.abs_path.display()
        );
        return Ok(());
    }

    if !&dotfile_path.abs_target.exists() {
        println!(
            "{} Repository path does not exist: {}",
            "✗".red(),
            &dotfile_path.abs_target.display()
        );
        return Ok(());
    }

    if dotfile_path.abs_path.is_dir() && dotfile_path.abs_target.is_dir() {
        return diff_directories(dotfile_path, tool);
    }

    // If an external diff tool is specified, use it for both files and directories
    if let Some(tool_name) = tool {
        return use_external_diff_tool(tool_name, &dotfile_path.abs_path, &dotfile_path.abs_target);
    }

    if !dotfile_path.abs_path.is_dir() && !&dotfile_path.abs_target.is_dir() {
        return diff_files(dotfile_path, word_diff);
    }

    println!(
        "{} Cannot compare: {} is a directory and {} is a file",
        "✗".red(),
        if dotfile_path.abs_target.is_dir() {
            "Repository path"
        } else {
            "Local path"
        },
        if dotfile_path.abs_target.is_dir() {
            "Local path"
        } else {
            "Repository path"
        }
    );

    Ok(())
}

fn diff_files(dotfile_path: DotPath, _word_diff: bool) -> Result<()> {
    let repo_file = &dotfile_path.abs_target;
    let local_file = &dotfile_path.abs_path;

    let local_content = fs::read_to_string(local_file)
        .with_context(|| format!("Failed to read local file: {}", local_file.display()))?;
    let repo_content = fs::read_to_string(repo_file)
        .with_context(|| format!("Failed to read repository file: {}", repo_file.display()))?;

    if local_content == repo_content {
        println!("{} Files are identical", "✓".green());
        return Ok(());
    }

    println!(
        "{} Differences between {} and repository version:",
        "✦".cyan(),
        dotfile_path.to_name().display()
    );

    // TODO: try to improve the diff output
    // if word_diff {
    //     let patch = diffy::create_patch(&repo_content, &local_content);
    //     print!("{}", patch);
    // } else {
    let patch = diffy::create_patch(&repo_content, &local_content);

    print!("{}", patch);
    // }

    Ok(())
}

fn diff_directories(dotfile_path: DotPath, tool: Option<String>) -> Result<()> {
    let repo_dir = &dotfile_path.abs_target;
    let local_dir = &dotfile_path.abs_path;

    if let Some(tool_name) = tool {
        return use_external_diff_tool(tool_name, repo_dir, local_dir);
    }

    println!(
        "{} Comparing directory: {}",
        "✦".cyan(),
        dotfile_path.to_name().display()
    );

    let differences = compare_directories(repo_dir, local_dir)?;

    if differences.is_empty() {
        println!("{} Directories are identical", "✓".green());
        return Ok(());
    }

    let mut sorted_diffs: Vec<(&PathBuf, &DiffStatus)> = differences.iter().collect();

    sorted_diffs.sort_by(|a, b| a.0.to_string_lossy().cmp(&b.0.to_string_lossy()));

    println!("{} Directory differences:", "✦".cyan());
    println!("{:4} | Path", "Type");
    println!(
        "{}",
        "--------------------------------------------------------------------------------".dimmed()
    );

    for (path, status) in sorted_diffs {
        let status_str = match status {
            DiffStatus::Added => "[+]".green(),
            DiffStatus::Modified => "[M]".yellow(),
            DiffStatus::Removed => "[-]".red(),
        };

        println!("{:4} | {}", status_str, path.display());
    }

    Ok(())
}

fn compare_directories(repo_dir: &Path, local_dir: &Path) -> Result<HashMap<PathBuf, DiffStatus>> {
    let mut differences = HashMap::new();
    let repo_files = collect_files(repo_dir, repo_dir)?;
    let local_files = collect_files(local_dir, local_dir)?;

    for (rel_path, repo_path) in &repo_files {
        let local_path = local_dir.join(rel_path);

        if !local_path.exists() {
            differences.insert(rel_path.clone(), DiffStatus::Removed);
        } else if local_path.is_file() && repo_path.is_file() {
            let repo_content = fs::read(repo_path)?;
            let local_content = fs::read(&local_path)?;

            if repo_content != local_content {
                differences.insert(rel_path.clone(), DiffStatus::Modified);
            }
        }
    }

    for rel_path in local_files.keys() {
        let repo_path = repo_dir.join(rel_path);

        if !repo_path.exists() {
            differences.insert(rel_path.clone(), DiffStatus::Added);
        }
    }

    Ok(differences)
}

fn collect_files(base_dir: &Path, current_dir: &Path) -> Result<HashMap<PathBuf, PathBuf>> {
    let mut files = HashMap::new();

    if !current_dir.exists() || !current_dir.is_dir() {
        return Ok(files);
    }

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        // Skip hidden files and directories
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with("."))
            .unwrap_or(false)
        {
            continue;
        }

        let relative_path = path.strip_prefix(base_dir)?.to_path_buf();

        if metadata.is_file() {
            files.insert(relative_path, path);
        } else if metadata.is_dir() {
            // Recursively collect files from subdirectories
            let mut subdir_files = collect_files(base_dir, &path)?;
            files.extend(subdir_files.drain());
        }
    }

    Ok(files)
}

fn use_external_diff_tool(tool_name: String, repo_path: &Path, local_path: &Path) -> Result<()> {
    let diff_cmd = match tool_name.as_str() {
        "vimdiff" => diff_tools::vimdiff_command(repo_path, local_path),
        "meld" => diff_tools::meld_command(repo_path, local_path),
        "kdiff3" => diff_tools::kdiff3_command(repo_path, local_path),
        "vscode" | "code" => diff_tools::vscode_command(repo_path, local_path),
        "diff" | "git-diff" => diff_command(repo_path, local_path),
        _ => return Err(anyhow!("Unsupported diff tool: {}", tool_name)),
    };

    match diff_cmd {
        Ok((cmd, args)) => {
            let status = Command::new(cmd)
                .args(args)
                .status()
                .with_context(|| format!("Failed to run diff tool: {}", tool_name))?;

            if !status.success() && !matches!(tool_name.as_str(), "diff" | "git-diff") {
                return Err(anyhow!(
                    "Diff tool process exited with non-zero status: {}",
                    status
                ));
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn diff_command(file1: &Path, file2: &Path) -> Result<(String, Vec<String>)> {
    let mut args = vec![
        "--color=always".to_string(),
        file1.to_string_lossy().to_string(),
        file2.to_string_lossy().to_string(),
    ];

    if file1.is_dir() && file2.is_dir() {
        args.insert(0, "-r".to_string());
    }

    Ok(("diff".to_string(), args))
}

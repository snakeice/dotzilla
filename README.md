# Dotzilla

A powerful, Git-inspired dotfiles manager with symbolic linking capabilities.

## Table of Contents

- [Dotzilla](#dotzilla)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Features](#features)
  - [Installation](#installation)
    - [From Source](#from-source)
    - [Using Cargo](#using-cargo)
  - [Usage](#usage)
    - [Launch Terminal User Interface](#launch-terminal-user-interface)
    - [Initialize a Repository](#initialize-a-repository)
    - [Adding Dotfiles](#adding-dotfiles)
    - [Removing Dotfiles](#removing-dotfiles)
    - [Staging Files](#staging-files)
    - [Unstaging Files](#unstaging-files)
    - [Creating Symlinks](#creating-symlinks)
    - [Viewing Status](#viewing-status)
    - [Listing Tracked Files](#listing-tracked-files)
    - [Comparing Files](#comparing-files)
  - [Command Reference](#command-reference)
  - [Example Workflow](#example-workflow)
  - [Shell Completion](#shell-completion)
  - [Changelog](#changelog)
  - [Contributing](#contributing)
  - [License](#license)

## Overview

Dotzilla is a Terminal User Interface (TUI) tool designed to help you manage your dotfiles with a workflow inspired by Git. It launches an interactive interface by default, and also provides command-line options for scripting and automation. It allows you to track, stage, and link dotfiles between a central repository and their original locations on your system.

## Features

- **Terminal User Interface**: Interactive TUI for easy dotfile management
- **Repository Management**: Initialize and maintain a centralized dotfiles repository
- **Tracking System**: Add and remove dotfiles from your repository
- **Staging Workflow**: Stage files before linking, similar to Git's staging area
- **Symbolic Linking**: Create symlinks from your repository to system locations
- **Diff Support**:
  - Compare tracked dotfiles with their system originals
  - Support for both file and directory diffs
  - Word-by-word diff option
  - Integration with external diff tools (vimdiff, meld, kdiff3, VS Code)
- **Status Reporting**: View the status of your tracked and staged dotfiles
- **Shell Completion**: Support for bash, zsh, fish, and other shells

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/snakeice/dotzilla.git
cd dotzilla

# Build the project
cargo build --release

# The binary will be available at target/release/dotzilla
# Optionally, move it to a directory in your PATH
cp target/release/dotzilla ~/.local/bin/
```

### Using Cargo

```bash
cargo install dotzilla-cli
```

## Usage

### Launch Terminal User Interface

The TUI launches by default when you run dotzilla without any commands:

```bash
dotzilla
```

You can also explicitly launch it with:

```bash
dotzilla tui
```

The TUI provides a visual interface for managing your dotfiles with the following features:

- **Tracked Tab**: View and manage all tracked dotfiles
- **Staged Tab**: View and manage staged dotfiles ready for linking
- **Help Tab**: View keyboard shortcuts and commands

**Keyboard Shortcuts:**

- `↑/k` or `↓/j` - Navigate up/down
- `Tab`/`Shift+Tab` - Switch between tabs
- `a` - Add a new dotfile
- `d` - Remove selected dotfile
- `s` - Stage/unstage selected dotfile
- `l` - Link all staged dotfiles
- `u` - Unlink all dotfiles
- `c` - Commit all staged dotfiles
- `r` - Refresh data
- `q` - Quit

### Initialize a Repository

Create a new dotfiles repository:

```bash
dotzilla init ~/dotfiles
```

### Adding Dotfiles

Add a dotfile to tracking:

```bash
dotzilla add ~/.bashrc
```

This copies the file to your dotfiles repository and begins tracking it.

### Removing Dotfiles

Remove a dotfile from tracking:

```bash
dotzilla remove .bashrc
```

This removes the dotfile from tracking and by default will ask for confirmation before deleting the file from the repository. Use the `--keep` flag to only remove from tracking without deleting the file:

```bash
dotzilla remove .bashrc --keep
```

### Staging Files

Stage a dotfile for linking:

```bash
dotzilla stage .bashrc
```

### Unstaging Files

Remove a file from the staging area:

```bash
dotzilla unstage .bashrc
```

### Creating Symlinks

Link all staged dotfiles to their original locations:

```bash
dotzilla link
```

This creates symbolic links from your dotfiles repository to the original locations.

### Viewing Status

Check the status of your dotfiles:

```bash
dotzilla status
```

### Listing Tracked Files

List all tracked dotfiles:

```bash
dotzilla list
```

### Comparing Files

Compare a tracked dotfile with its system original:

```bash
# Basic diff
dotzilla diff .bashrc

# Word-by-word diff
dotzilla diff .bashrc --word

# Using an external diff tool
dotzilla diff .bashrc --tool vimdiff
dotzilla diff .bashrc --tool meld
dotzilla diff .bashrc --tool vscode

# Directory diff
dotzilla diff .config
```

## Command Reference

| Command                                | Description                                         |
| -------------------------------------- | --------------------------------------------------- |
| `tui`                                  | Launch the Terminal User Interface                  |
| `init [path]`                          | Initialize a new dotfiles repository                |
| `add <path>`                           | Add a dotfile to tracking                           |
| `remove <name> [--keep]`               | Remove a dotfile from tracking                      |
| `stage <name>`                         | Stage a dotfile for linking                         |
| `unstage <name>`                       | Unstage a dotfile                                   |
| `link`                                 | Link all staged dotfiles to their target locations  |
| `status`                               | Show the status of tracked and staged dotfiles      |
| `list`                                 | List all tracked dotfiles                           |
| `diff <name> [--word] [--tool <tool>]` | Show differences between tracked and local dotfiles |
| `completion <shell>`                   | Generate shell completion scripts                   |

## Example Workflow

### Using the Command Line

1. Initialize your dotfiles repository:

   ```bash
   dotzilla init ~/dotfiles
   ```

2. Add some dotfiles:

   ```bash
   dotzilla add ~/.bashrc
   dotzilla add ~/.vimrc
   dotzilla add ~/.config/nvim
   ```

3. Check their status:

   ```bash
   dotzilla status
   ```

4. Stage files you want to link:

   ```bash
   dotzilla stage .bashrc
   dotzilla stage .vimrc
   ```

5. Create symlinks:

   ```bash
   dotzilla link
   ```

6. Later, if you make changes to files in their original locations, compare them:

   ```bash
   dotzilla diff .bashrc
   ```

7. Add the updated file to incorporate changes:

   ```bash
   dotzilla add ~/.bashrc
   ```

8. If you no longer need to track a dotfile, remove it:

   ```bash
   # Remove from tracking and delete the file (with confirmation)
   dotzilla remove .vimrc

   # Or just remove from tracking, keeping the file
   dotzilla remove .vimrc --keep
   ```

### Using the TUI

Alternatively, you can use the interactive Terminal User Interface for a more visual experience. The TUI is the default interface and launches automatically:

1. Launch the TUI (default behavior):

   ```bash
   dotzilla
   ```

   Or explicitly:

   ```bash
   dotzilla tui
   ```

2. Use the keyboard shortcuts to navigate and manage your dotfiles:
   - Navigate between tracked and staged dotfiles using `Tab`
   - Add new dotfiles with `a`
   - Stage/unstage files with `s`
   - Link all staged files with `l`
   - Press `?` or go to the Help tab for a complete list of shortcuts

## Shell Completion

Dotzilla supports shell completion for various shells, making it easier to use the command-line interface.

Generate shell completion scripts:

```bash
# For Bash
dotzilla completion bash > ~/.local/share/bash-completion/completions/dotzilla

# For Zsh
dotzilla completion zsh > ~/.zfunc/_dotzilla

# For Fish
dotzilla completion fish > ~/.config/fish/completions/dotzilla.fish
```

Make sure the respective directories exist and are in your shell's completion path.

## Changelog

See the [CHANGELOG](./CHANGELOG) file for a detailed history of changes.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

# Dotzilla

A powerful, Git-inspired dotfiles manager with symbolic linking capabilities.

## Overview

Dotzilla is a command-line tool designed to help you manage your dotfiles with a workflow inspired by Git. It allows you to track, stage, and link dotfiles between a central repository and their original locations on your system.

## Features

- **Repository Management**: Initialize and maintain a centralized dotfiles repository
- **Tracking System**: Add and track dotfiles in your repository
- **Staging Workflow**: Stage files before linking, similar to Git's staging area
- **Symbolic Linking**: Create symlinks from your repository to system locations
- **Diff Support**:
  - Compare tracked dotfiles with their system originals
  - Support for both file and directory diffs
  - Word-by-word diff option
  - Integration with external diff tools (vimdiff, meld, kdiff3, VS Code)
- **Status Reporting**: View the status of your tracked and staged dotfiles

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/dotzilla.git
cd dotzilla

# Build the project
cargo build --release

# The binary will be available at target/release/dotzilla
# Optionally, move it to a directory in your PATH
cp target/release/dotzilla ~/.local/bin/
```

## Usage

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
| `init [path]`                          | Initialize a new dotfiles repository                |
| `add <path>`                           | Add a dotfile to tracking                           |
| `stage <name>`                         | Stage a dotfile for linking                         |
| `unstage <name>`                       | Unstage a dotfile                                   |
| `link`                                 | Link all staged dotfiles to their target locations  |
| `status`                               | Show the status of tracked and staged dotfiles      |
| `list`                                 | List all tracked dotfiles                           |
| `diff <name> [--word] [--tool <tool>]` | Show differences between tracked and local dotfiles |

## Example Workflow

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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

# 🔥 GORNILO 🔥

## GORNILO is a build tool for [Odin](odin-lang.org) projects

## Installation
```bash
cargo install gornilo
```

## Usage
### Create a project
```bash
gornilo new my_project --no-git --no-ols --no-mem-tracking --no-workflows
```
### Build the project
```bash
gornilo build --release
```
### Run the project
```bash
gornilo run --release
```

### Clean temporary files
```bash
gornilo clean
```

## Configuration
GORNILO has a configuration file `gornilo.toml` which is created in the project's root.<br>
Available options:
- `name` - project name

## Why Rust?
I wanted to implement GORNILO in Odin, but Odin's standard library has a very poor support for executing command line commands.

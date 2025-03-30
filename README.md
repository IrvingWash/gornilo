# 🔥GORNILO🔥

## GORNILO is a project management tool for [Odin](https://odin-lang.org)

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

### Run an example
```bash
gornilo run --release --example my_example
```

### Clean temporary files
```bash
gornilo clean
```

## Configuration
GORNILO has a configuration file `gornilo.toml` which is created in the project's root.<br>
Available options:<br>
```toml
[project]
name = "my_project"

[vet_flags]
warnings_as_errors = true
unused_variables = true
unused_imports = true
tabs = true
style = true
semicolon = true
cast = true
```

## Examples
Examples should have the following structure:
```
my_project
├── examples
│   └── my_example
│       └── src
│           └── main.odin
```

## Todo
- Retain `odin build`'s output formatting
- Add ability to run tests

## Why Rust?
I wanted to implement GORNILO in Odin, but Odin's standard library has a very poor support for executing command line commands.

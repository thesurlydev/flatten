# Flattener

A Rust CLI tool that flattens nested project directories into a single directory with underscore-delimited filenames 
while preserving the original structure.

## Features

- Preserves original project structure
- Respects .gitignore and .flattenignore patterns
- Handles filename collisions automatically
- Processes hidden files
- Creates underscore-delimited filenames based on original paths
- Automatically removes existing 'flattened' directory if present

## Error Handling

- Validates directory existence
- Reports ignored files
- Shows detailed progress during processing
- Reports total number of processed files

## Installation

```bash
cargo install flattener
```

## Usage

Run in the current directory:
```bash
flattener
```

Or specify a target directory:
```bash
flattener /path/to/project
```

## Example

Given this project structure:
```
my-project/
├── src/
│   ├── main.rs
│   └── utils/
│       ├── helpers.rs
│       └── config/
│           └── settings.rs
├── tests/
│   └── integration.rs
└── Cargo.toml
```

Running `flattener` will create:
```
my-project/
├── [original structure remains]
└── flattened/
    ├── src_main.rs
    ├── src_utils_helpers.rs
    ├── src_utils_config_settings.rs
    ├── tests_integration.rs
    └── Cargo.toml
```

## Ignore Patterns

The tool respects both `.gitignore` and `.flattenignore` files. Create a `.flattenignore` file to specify additional patterns for files you don't want to include in the flattened output:

```gitignore
# .flattenignore
Cargo.lock
*.tmp
build/
```


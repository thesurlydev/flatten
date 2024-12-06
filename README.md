# flatten-project

A Rust CLI tool that flattens nested project directories into a single directory with underscore-delimited filenames 
while preserving the original structure.

## Features

- Preserves original project structure
- Respects `.gitignore` and `.flatignore` patterns
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

Clone the project and run:
```bash
cargo install --path .
```

## Usage

Run in the current directory:
```bash
flatten-project
```

Or specify a target directory:
```bash
flatten-project /path/to/project
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

Running `flatten` will create:
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

The tool respects both `.gitignore` and `.flatignore` files. Create a `.flatignore` file to specify additional patterns for files you don't want to include in the flattened output:

```gitignore
# .flatignore
Cargo.lock
*.tmp
build/
```



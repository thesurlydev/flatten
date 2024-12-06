
# Optional for all receipes except tag
TAG := ''

_validate-new-tag TAG:
    #!/usr/bin/env bash
    if ! [[ "{{TAG}}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Error: TAG must be in the format v0.0.0"
        exit 1
    fi
    if git rev-parse "{{TAG}}" >/dev/null 2>&1; then
        echo "Error: Tag {{TAG}} already exists"
        exit 1
    fi
    echo "Tagging {{TAG}}"

# Clean the project
clean:
    cargo clean

# Build the project
build: clean
    cargo build

# Run the project
run:
    cargo run

# Installs the project
install:
    cargo install --path .

# Tag with the given TAG arg and push
tag TAG: (_validate-new-tag TAG)
    git tag -a {{TAG}} -m "{{TAG}}"
    git push origin --tags

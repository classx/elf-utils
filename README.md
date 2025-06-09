# ELF Utils

A command-line utility for working with ELF (Executable and Linkable Format) files. This tool provides functionality to list and execute commands on ELF files within specified directories.

## Features

- Recursively find ELF files in directories
- List all discovered ELF files
- Execute commands on found ELF files
- Silent mode operation
- Exit code status for script integration

## Installation

To build the project, you'll need Rust and Cargo installed on your system. Then:

```bash
git clone <repository-url>
cd elf-utils
cargo build --release
```

The compiled binary will be available in `target/release/elf-utils`.

## Usage

### List Command

The `list` command finds and displays all ELF files in a specified directory and its subdirectories.

```bash
# List all ELF files in a directory
elf-utils list /usr/lib

# List ELF files silently (only returns exit code)
elf-utils list --silent /usr/lib
```

Exit codes:
- 0: ELF files were found
- 1: No ELF files were found

### Exec Command

The `exec` command executes a specified command on all found ELF files.

```bash
# Print file information for all ELF files
elf-utils exec /usr/lib "file"

# Get symbols from all ELF files silently
elf-utils exec --silent /usr/lib "nm"

# Check dependencies of ELF files
elf-utils exec /usr/lib "ldd"
```

Exit codes:
- 0: Command executed successfully on at least one ELF file
- 1: No ELF files found to execute command on

## Examples

1. Find all ELF files in /usr/local/bin:
```bash
elf-utils list /usr/local/bin
```

2. Check file type of all ELF files in a directory:
```bash
elf-utils exec /usr/local/lib "file"
```

3. List dependencies of all ELF files silently:
```bash
elf-utils exec --silent /usr/local/bin "ldd"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
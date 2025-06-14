# ELF Utils

A command-line utility for working with ELF (Executable and Linkable Format) files. This tool provides functionality to list and execute commands on ELF files within specified directories.

_For Linux x86 only._

## Features

- Recursively find ELF files in directories
- List all discovered ELF files
- Execute commands on found ELF files
- Check if a file is in ELF format
- Silent mode operation (--silent flag)
- Exit code status for script integration (0 for success, 1 for failure)
- Recursive directory scanning

## Installation

To build the project, you'll need Rust and Cargo installed on your system. Then:

```bash
git clone <repository-url>
cd elf-utils
make build
```

This will build a statically linked binary using musl target, which will be available at `target/x86_64-unknown-linux-musl/release/elf-utils`.

Or build using cargo directly for your native target:

```bash
cargo build --release
```

## Building From Source

You can use the provided Makefile which includes the following targets:

- `make build`: Build release binary with musl target
- `make test`: Run tests
- `make deb`: Create Debian package
- `make rpm`: Create RPM package
- `make clean`: Clean build artifacts

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
- 0: ELF files were found and processed successfully
- 1: No ELF files were found or operation failed

### Exec Command

The `exec` command executes a specified command on all found ELF files.

```bash
# Print file information for all ELF files
elf-utils exec /usr/lib "file"

# Get symbols from all ELF files silently
elf-utils exec --silent /usr/lib "nm"

# Check dependencies of ELF files
elf-utils exec /usr/lib "ldd"

The command will be executed on each ELF file found in the directory and its subdirectories. The file path is automatically appended as the last argument to your command.
```

Exit codes:
- 0: Command executed successfully on at least one ELF file
- 1: No ELF files found or command execution failed

### Check Command

The `check` command verifies if a specified file is in ELF format.

```bash
# Check if a file is in ELF format
elf-utils check /path/to/binary
```

Exit codes:
- 0: The file is an ELF file
- 1: The file is not an ELF file or doesn't exist

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

4. Verify if a file is an ELF binary:
```bash
elf-utils check /usr/bin/ls
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

use clap::{Parser, Subcommand};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about = "ELF utilities")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List information from ELF files
    List {
        /// Path to find ELF files
        #[clap(value_parser)]
        dir: String,

        /// Optional silent mode, only return exit code
        #[clap(long, short, default_value_t = false)]
        silent: bool,
    },

    /// Execute a command on an ELF file
    Exec {
        /// Path to the ELF file
        #[clap(value_parser)]
        dir: String,
        /// Command to execute on the ELF file
        #[clap(value_parser)]
        command: String,

        /// Optional silent mode, only return exit code
        #[clap(long, short, default_value_t = false)]
        silent: bool,
    },
    /// Check if the file is an ELF file, only return exit code
    Check {
        /// Path to the ELF file
        #[clap(value_parser)]
        filename: String,
    },

}


struct Libdir {
    dir: String,
    files: Vec<Files>,
}

struct Files {
    fullpath: String,
}

impl Libdir {
    fn new(dirname: String) -> Self {
        Self {
            dir: dirname,
            files: Vec::new(),
        }
    }

    fn populate_files(&mut self) {
        let dir_path = self.dir.clone();  // Clone the string to avoid borrowing issues
        self.scan_directory(&dir_path);
    }

    fn scan_directory(&mut self, dir_path: &str) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        // Recursively scan subdirectories
                        if let Some(path_str) = path.to_str() {
                            let path_string = path_str.to_string();  // Create owned string
                            self.scan_directory(&path_string);
                        }
                    } else if path.is_file() && !path.is_symlink() {
                        // Check if it's a regular file (not a symlink) and an ELF file
                        if let Ok(metadata) = fs::metadata(&path) {
                            if metadata.is_file() && is_elf(&path) {
                                self.files.push(Files {
                                    fullpath: path.to_string_lossy().to_string(),
                            });
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_elf(path: &Path) -> bool {
    if let Ok(mut file) = fs::File::open(path) {
        let mut magic = [0u8; 4];
        if let Ok(_) = file.read_exact(&mut magic) {
            return magic == [0x7f, b'E', b'L', b'F'];
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { dir, silent } => {
            // Implement list functionality here
            let mut dir = Libdir::new(dir.to_string());
            dir.populate_files();
            if !dir.files.is_empty() {
                if !*silent {
                    for file in dir.files {
                        println!("{}", file.fullpath);
                    }
                }
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }

        Commands::Exec { dir, command, silent } => {
            // Implement exec functionality here
            let mut dir = Libdir::new(dir.to_string());
            dir.populate_files();
            if !dir.files.is_empty() {
                for file in dir.files {
                    //println!("Executing command: {} on file: {}", command, file.fullpath);
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    let cmd = parts[0];
                    let mut cmd_process = std::process::Command::new(cmd);
                    // Add any arguments from the command string
                    for arg in &parts[1..] {
                        cmd_process.arg(arg);
                    }
                    // Add the file path as the final argument
                    cmd_process.arg(&file.fullpath);
                    let output = cmd_process.output().expect("Failed to execute command");
                    if !silent {
                        if output.status.success() {
                            println!("{}", String::from_utf8_lossy(&output.stdout).trim());
                        } else {
                            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr).trim());
                        }
                    }
                }
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }

        Commands::Check { filename } => {
            // Implement check functionality here
            let path = Path::new(filename);
            if is_elf(path) {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }

    }
}

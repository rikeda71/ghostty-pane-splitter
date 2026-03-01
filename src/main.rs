mod config;
mod keybind;
mod layout;
mod split;

use config::{find_config_path, parse_keybindings};
use layout::parse_layout;
use split::execute_splits;

/// Application version string loaded from Cargo.toml.
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "--help" | "-h" => print_usage(),
        "--version" | "-V" => println!("ghostty-pane-splitter {}", VERSION),
        arg => match parse_layout(arg) {
            Ok(layout) => {
                let config_path = match find_config_path() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                };
                let content = match std::fs::read_to_string(&config_path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error: Failed to read {}: {}", config_path.display(), e);
                        std::process::exit(1);
                    }
                };
                let keybindings = match parse_keybindings(&content) {
                    Ok(k) => k,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                };
                // Uniform grid if all columns have the same row count
                let is_uniform = layout.columns.windows(2).all(|w| w[0] == w[1]);
                if is_uniform {
                    println!(
                        "Splitting into {}x{} grid ({} panes)...",
                        layout.num_cols(),
                        layout.columns[0],
                        layout.total_panes()
                    );
                } else {
                    let cols_str: Vec<String> =
                        layout.columns.iter().map(|c| c.to_string()).collect();
                    println!(
                        "Splitting into custom layout {} ({} panes)...",
                        cols_str.join(","),
                        layout.total_panes()
                    );
                }
                if let Err(e) = execute_splits(&keybindings, &layout) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                println!("Done!");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}

fn print_usage() {
    println!(
        "ghostty-pane-splitter {}
CLI tool to split panes on Ghostty Terminal

USAGE:
    ghostty-pane-splitter <LAYOUT>

ARGS:
    <LAYOUT>    Number of panes (e.g. 4), grid spec (e.g. 2x3), or custom layout (e.g. 1,3)

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

EXAMPLES:
    ghostty-pane-splitter 4      # Split into 2x2 grid
    ghostty-pane-splitter 2x3    # Split into 2 cols x 3 rows
    ghostty-pane-splitter 1,3    # Left: 1 pane, Right: 3 panes
    ghostty-pane-splitter 2,1,3  # 3 columns with 2, 1, 3 rows",
        VERSION
    );
}

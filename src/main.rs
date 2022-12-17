use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to raad
    path: std::path::PathBuf,
}

fn main() {
    let _args = Args::parse();
}

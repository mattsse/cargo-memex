use crate::meme::MemeFile;
use std::path::PathBuf;
use structopt::StructOpt;

/// Executes build of the memex executable meme which produces meme "binary".
///
/// It does so by invoking `cargo build` and then post processing the final binary.
#[derive(Debug, StructOpt)]
#[structopt(name = "build")]
pub struct BuildCommand {
    /// Path to the Cargo.toml of the cargo project
    #[structopt(long, parse(from_os_str))]
    manifest_path: Option<PathBuf>,

    /// The targeted meme
    #[structopt(parse(try_from_str))]
    meme: MemeFile,

    /// Build the specified binary
    #[structopt(long)]
    bin: Option<String>,

    /// Build the meme in release mode, with optimizations
    #[structopt(long)]
    release: bool,
}

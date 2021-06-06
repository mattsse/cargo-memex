use crate::build::BuildCommand;
use std::path::PathBuf;
use structopt::StructOpt;

/// Similarly to cargo run, but with memes
#[derive(Debug, StructOpt)]
#[structopt(name = "build")]
pub struct RunCommand {
    #[structopt(flatten)]
    build: BuildCommand,
}

use memex::build::BuildCommand;
use memex::run::RunCommand;
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub(crate) enum Opts {
    /// Utilities to compile memes.
    #[structopt(name = "memex")]
    Memex(MemexArgs),
}

#[derive(Debug, StructOpt)]
pub(crate) struct MemexArgs {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Compiles the project into a meme
    #[structopt(name = "build")]
    Build(BuildCommand),
    /// Executes a meme
    #[structopt(name = "run")]
    Run(RunCommand),
}

fn main() {
    let Opts::Memex(args) = Opts::from_args();
}

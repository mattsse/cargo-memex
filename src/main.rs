use cargo_memex::build::BuildCommand;
use cargo_memex::exec::ExecCommand;
use cargo_memex::run::RunCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub(crate) enum Opts {
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
    /// Builds and runs a meme
    #[structopt(name = "run")]
    Run(RunCommand),
    /// Executes a meme
    #[structopt(name = "exec")]
    Exec(ExecCommand),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let Opts::Memex(args) = Opts::from_args();
    match args.cmd {
        Command::Build(cmd) => {
            cmd.run()?;
        }
        Command::Run(cmd) => cmd.run()?,
        Command::Exec(cmd) => cmd.run()?,
    }
    Ok(())
}

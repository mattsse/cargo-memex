use memex::build::BuildCommand;
use memex::exec::ExecCommand;
use memex::run::RunCommand;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
enum Opts {
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
    let cmd = Opts::from_args();
    match cmd {
        Opts::Build(cmd) => {
            cmd.run()?;
        }
        Opts::Run(cmd) => cmd.run()?,
        Opts::Exec(cmd) => cmd.run()?,
    }
    Ok(())
}

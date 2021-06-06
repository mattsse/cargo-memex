use crate::build::BuildCommand;
use crate::exec::ExecCommand;
use structopt::StructOpt;

/// Similarly to cargo run, but with memes
#[derive(Debug, StructOpt)]
#[structopt(name = "run")]
pub struct RunCommand {
    #[structopt(flatten)]
    build: BuildCommand,
}

impl RunCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let output = self.build.run()?;
        ExecCommand::new(output.meme_path).run()
    }
}

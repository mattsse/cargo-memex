use crate::meme::Meme;
use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;

/// Executes a memex meme
#[derive(Debug, StructOpt)]
#[structopt(name = "exec")]
pub struct ExecCommand {
    #[structopt(parse(from_os_str))]
    meme: PathBuf,
}

impl ExecCommand {
    pub fn new(meme: impl AsRef<Path>) -> Self {
        Self {
            meme: meme.as_ref().to_path_buf(),
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut meme_exe = self.meme.clone();
        meme_exe.set_extension("meme");

        Meme::decode_bin_to(&self.meme, &meme_exe)?;

        let mut cmd = Command::new(format!("./{}", meme_exe.display()));
        log::debug!("Executing meme `{:?}`", cmd);
        let child = cmd.spawn()?;
        child
            .wait_with_output()
            .context(format!("Error executing `{:?}`", cmd))?;
        let _ = std::fs::remove_file(meme_exe);
        Ok(())
    }
}

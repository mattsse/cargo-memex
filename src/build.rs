use crate::manifest::Manifest;
use crate::meme::Meme;
use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;
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
    meme: String,

    /// Build the specified binary
    #[structopt(long)]
    bin: Option<String>,

    /// Build the specified examples
    #[structopt(long)]
    example: Option<String>,

    /// Build the meme in release mode, with optimizations
    #[structopt(long)]
    release: bool,
}

impl BuildCommand {
    /// execute the build command
    pub fn run(&self) -> anyhow::Result<BuildOutput> {
        let meme = Meme::new(&self.meme)?;
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let mut cmd = Command::new(cargo);
        cmd.arg("build");

        let (manifest, mut bin_path) = if let Some(ref path) = self.manifest_path {
            cmd.arg("--manifest-path").arg(path);
            (
                Manifest::new(path)?,
                path.parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from(".")),
            )
        } else {
            (Manifest::new("./Cargo.toml")?, PathBuf::from("."))
        };
        bin_path.push("target");

        if self.release {
            cmd.arg("--release");
            bin_path.push("release");
        } else {
            bin_path.push("debug");
        }

        let bin_name = if let Some(ref bin) = self.bin {
            cmd.arg("--bin").arg(bin);
            bin.clone()
        } else if let Some(ref example) = self.example {
            cmd.arg("--example").arg(example);
            bin_path.push("examples");
            example.clone()
        } else {
            manifest.name()?.to_string()
        };
        bin_path.push(&bin_name);

        log::debug!("Executing: `{:?}`", cmd);
        let child = cmd.spawn()?;
        let output = child
            .wait_with_output()
            .context(format!("Error executing `{:?}`", cmd))?;

        if !output.status.success() {
            anyhow::bail!(
                "`{:?}` failed with exit code: {:?}",
                cmd,
                output.status.code()
            );
        }

        let mut meme_path = bin_path.clone();
        meme_path.set_extension("jpeg");
        meme.write_with_bin_to(&bin_path, &meme_path)?;
        Ok(BuildOutput {
            meme_path,
            bin_path,
            bin_name,
        })
    }
}

pub struct BuildOutput {
    pub meme_path: PathBuf,
    /// Path to the cargo binary.
    pub bin_path: PathBuf,
    /// Name of the executable
    pub bin_name: String,
}

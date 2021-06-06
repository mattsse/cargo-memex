use anyhow::Context;
use std::path::{Path, PathBuf};
use toml::value;

pub struct Manifest {
    pub manifest_path: PathBuf,
    pub cargo_toml: value::Table,
}

impl Manifest {
    pub fn new(manifest_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let manifest_path = manifest_path.as_ref().to_path_buf();
        let toml = std::fs::read_to_string(manifest_path.as_path())?;
        log::debug!("Parsing toml file {}", manifest_path.display());
        let cargo_toml = toml::from_str::<value::Table>(&toml)?;
        Ok(Manifest {
            manifest_path,
            cargo_toml,
        })
    }

    pub fn name(&self) -> anyhow::Result<&str> {
        self.cargo_toml
            .get("package")
            .context("package not found in toml")?
            .get("name")
            .context("package.name field not found in toml")?
            .as_str()
            .context("package.name field must be a string")
    }
}

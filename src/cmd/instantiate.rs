use anyhow::Result;

#[derive(Debug, clap::Args)]
pub struct InstantiateCommand {}

impl InstantiateCommand {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
use anyhow::{Result, Ok};

#[derive(Debug, clap::Args)]
#[clap(
    name = "hello",
    about = "Greeting!"
)]
pub struct HelloCommand {
    #[clap(short, long)]
    name: String,
}

impl HelloCommand {
    pub fn exec(&self) -> Result<()> {
        println!("Hello {}", self.name);
        Ok(())
    }
}

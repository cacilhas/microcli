use clap::Parser;
use eyre::Result;
use kodumaro_http_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::try_parse()?;
    kodumaro_http_cli::perform(args).await?;
    Ok(())
}

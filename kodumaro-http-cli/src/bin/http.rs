use clap::Parser;
use eyre::Result;
use kodumaro_http_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cl_parameters = Cli::parse();
    cl_parameters.initialize()?;
    kodumaro_http_cli::perform(cl_parameters).await?;
    Ok(())
}

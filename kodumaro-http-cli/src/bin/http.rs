use clap::{error::ErrorKind, Parser};
use eyre::Result;
use kodumaro_http_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cl_parameters = match Cli::try_parse() {
        Ok(args) => args,
        Err(err) if [ErrorKind::DisplayHelp, ErrorKind::DisplayVersion].contains(&err.kind()) => {
            eprintln!("{}", err);
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };
    cl_parameters.initialize()?;
    kodumaro_http_cli::perform(cl_parameters).await?;
    Ok(())
}

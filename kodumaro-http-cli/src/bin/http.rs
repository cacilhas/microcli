use clap::{error::ErrorKind, Parser};
use eyre::Result;
use kodumaro_http_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(err) if [ErrorKind::DisplayHelp, ErrorKind::DisplayVersion].contains(&err.kind()) => {
            eprintln!("{}", err);
            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };
    kodumaro_http_cli::perform(args).await?;
    Ok(())
}

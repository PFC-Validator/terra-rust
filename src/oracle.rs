use structopt::StructOpt;
use terra_rust_api::Terra;

use crate::errors::Result;

#[derive(StructOpt)]
pub enum OracleCommand {
    #[structopt(name = "parameters", about = "Get Oracle Parameters")]
    Parameters,
}

pub async fn oracle_cmd_parse(
    terra: &Terra<'_>,
    _wallet: &str,
    _seed: Option<&str>,
    oracle_cmd: OracleCommand,
) -> Result<()> {
    match oracle_cmd {
        OracleCommand::Parameters => {
            let resp = terra.oracle().parameters().await?;

            println!("{}", serde_json::to_string(&resp)?)
        }
    }
    Ok(())
}

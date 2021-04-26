use crate::client::oracle_types::OracleParametersResult;
use crate::errors::Result;
use crate::Terra;

pub struct Oracle<'a> {
    terra: &'a Terra<'a>,
}
impl Oracle<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Oracle<'a> {
        Oracle { terra }
    }
    pub async fn parameters(&self) -> Result<OracleParametersResult> {
        let response = self
            .terra
            .send_cmd::<OracleParametersResult>("/oracle/parameters", None)
            .await?;
        Ok(response)
    }
}

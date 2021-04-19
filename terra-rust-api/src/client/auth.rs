use crate::client::auth_types::AuthAccountResult;
use crate::errors::Result;
use crate::Terra;

pub struct Auth<'a> {
    terra: &'a Terra<'a>,
}
impl Auth<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Auth<'a> {
        Auth { terra }
    }
    pub async fn account(&self, account_address: &str) -> Result<AuthAccountResult> {
        let response = self
            .terra
            .send_cmd::<AuthAccountResult>(&format!("/auth/accounts/{}", account_address), None)
            .await?;
        Ok(response)
    }
}

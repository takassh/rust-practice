use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

pub async fn get_endpoint_list() -> Result<ListResponse> {
    let response = reqwest::get("https://pokeapi.co/api/v2/").await?;
    let json = response.text().await.map_err(|e| Error::new(e))?;

    serde_json::from_str(&json).map_err(|e| Error::new(e))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ListResponse {
    pub ability: String,
    pub berry: Option<String>,
    pub berry_firmness: String,
    // ... and so on
}

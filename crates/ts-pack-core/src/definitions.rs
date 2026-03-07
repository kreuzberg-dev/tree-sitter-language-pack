use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanguageDefinition {
    pub repo: String,
    #[serde(default)]
    pub rev: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub generate: Option<bool>,
    #[serde(default)]
    pub abi_version: Option<u32>,
}

pub type LanguageDefinitions = BTreeMap<String, LanguageDefinition>;

pub fn load_definitions(json: &str) -> Result<LanguageDefinitions, serde_json::Error> {
    serde_json::from_str(json)
}

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub folders: Vec<NamespacePattern>,

    #[serde(default)]
    pub files: Vec<NamespacePattern>,

    #[serde(default)]
    pub rules: Vec<NamespaceRules>,
}

#[derive(Debug, Deserialize)]
pub struct NamespacePattern {
    pub namespace: String,
    pub patterns: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct NamespaceRules {
    pub namespaces: Vec<String>,
    #[serde(rename = "allowImports")]
    pub allow_imports: Vec<String>,
}

pub fn load_config(root: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = root.join("depend.json");
    let contents = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}

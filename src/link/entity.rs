use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub path: String,
    pub to: String
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub config: Config
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub to: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub config: Config
}

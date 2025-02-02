use std::fs;
use crate::link::entity::Link;

pub fn parse(id: String) -> Result<Link, String> {
    let path = format!("links/{}.toml", id);
    let string = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(error) => return Err(error.to_string())
    };

    match toml::from_str::<Link>(&string) {
        Ok(link) => Ok(link),
        Err(error) => Err(error.to_string())
    }
}

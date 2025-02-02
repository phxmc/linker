use std::fs;
use toml::from_str;
use crate::reference::entity::Reference;

pub fn get(id: String) -> Result<Reference, String> {
    let path = format!("refs/{}.toml", id);

    let toml = match fs::read_to_string(path) {
        Ok(toml) => toml,
        Err(error) => return Err(error.to_string())
    };

    match from_str(&toml) {
        Ok(reference) => Ok(reference),
        Err(error) => Err(error.to_string())
    }
}

pub fn set(id: String, reference: Reference) -> Result<(), String> {
    let path = format!("refs/{}.toml", id);

    let toml = match toml::to_string(&reference) {
        Ok(toml) => toml,
        Err(error) => return Err(error.to_string())
    };

    match fs::write(path, toml) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string())
    }
}

use std::{fs, io};
use crate::link::entity::Link;

pub fn parse_links() -> Result<Vec<Link>, io::Error> {
    let mut links: Vec<Link> = vec![];
    for entry in fs::read_dir("links")? {
        let entry = entry?;
        let file_name = entry.file_name();

        let file_name_str = match file_name.into_string() {
            Ok(name) => name,
            Err(_) => continue
        };

        let is_toml = file_name_str.ends_with(".toml");
        if !is_toml {
            continue;
        }

        match parse_by_name(&file_name_str) {
            Ok(link) => links.push(link),
            Err(e) => eprintln!("Ошибка парсинга файла {}: {}", file_name_str, e)
        }
    }

    Ok(links)
}

pub fn parse_by_name(name: &String) -> Result<Link, String> {
    let path = format!("links/{}", name);
    let string = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(error) => return Err(error.to_string())
    };

    match toml::from_str::<Link>(&string) {
        Ok(link) => Ok(link),
        Err(error) => Err(error.to_string())
    }
}

pub fn parse_by_id(id: &String) -> Result<Link, String> {
    let name = format!("{}.toml", id);
    parse_by_name(&name)
}

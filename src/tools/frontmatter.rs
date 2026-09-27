use std::{error::Error, io::ErrorKind};

use serde::de::DeserializeOwned;

const DELIMITER: &str = "---";

pub fn to_frontmatter_markdown<T: serde::Serialize>(
    header: &T,
    body: &String,
) -> Result<String, Box<dyn Error>> {
    let yaml = serde_yaml::to_string(header)?;
    let header = format!("{DELIMITER}\n{yaml}{DELIMITER}");

    let result = format!("{header}\n{body}");

    Ok(result)
}

pub fn from_frontmatter_markdown<H: DeserializeOwned>(
    data: &String,
) -> Result<(H, String), Box<dyn Error>> {
    let data = data
        .strip_prefix(DELIMITER)
        .ok_or(std::io::Error::from(ErrorKind::InvalidData))?;

    let (yaml, body) = data
        .split_once(DELIMITER)
        .ok_or(std::io::Error::from(ErrorKind::InvalidData))?;

    let header: H = serde_yaml::from_str(yaml)?;
    let body = body.strip_prefix('\n').unwrap_or(body).to_string();

    Ok((header, body))
}

use std::error::Error;

pub fn frontmatter_header<T: serde::Serialize>(object: &T) -> Result<String, Box<dyn Error>> {
    let yaml = serde_yaml::to_string(object)?;

    let result = format!("---\n{yaml}---");

    Ok(result)
}

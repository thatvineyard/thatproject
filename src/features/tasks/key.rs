use std::error::Error;

use crate::{app_context::AppContext, features::tasks::store};

const DELIMITER: &str = ":";

#[derive(Debug)]
pub struct Key {
    category: String,
    ordinal: u32,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:03}", self.category, DELIMITER, self.ordinal)
    }
}

impl std::str::FromStr for Key {
    type Err = Box<dyn Error>;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        if let Some((cat, ord)) = key.split_once(DELIMITER) {
            if let Ok(ordinal) = ord.parse::<u32>() {
                return Ok(Self {
                    category: cat.to_string(),
                    ordinal,
                });
            }
        }
        Err("Could not parse string to key".into())
    }
}

impl Key {
    pub fn next_key(context: &AppContext, category: String) -> Result<Self, Box<dyn Error>> {
        let tasks = store::list(context)?;

        let max_ordinal = tasks
            .iter()
            .filter(|task| task.header.key.category == category)
            .map(|task| task.header.key.ordinal)
            .max()
            .unwrap_or(0);
        let next_ordinal = max_ordinal + 1;

        Ok(Self {
            category,
            ordinal: next_ordinal,
        })
    }
}

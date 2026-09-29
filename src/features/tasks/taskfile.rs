use crate::app_context::AppContext;
use crate::features::tasks::key::{self, Key};
use crate::tools::{self, frontmatter};
use core::fmt;
use serde;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use std::error::Error;

const DEFAULT_CATEGORY: &str = "misc";

fn default_category() -> Category {
    Category::new(DEFAULT_CATEGORY).expect("DEFAULT_CATEGORY constant must be a valid category")
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Draft,
    Ongoing,
    Complete,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Draft => "Draft",
            Self::Ongoing => "Ongoing",
            Self::Complete => "Complete",
        };

        f.write_str(label)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFile {
    pub header: TaskFileHeader,
    pub body: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFileHeader {
    #[serde_as(as = "DisplayFromStr")]
    pub key: Key,
    pub title: String,
    #[serde(default = "default_category")]
    pub category: Category,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Category(String);

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Category {
    pub fn new(s: &str) -> Result<Self, Box<dyn Error>> {
        if s.contains(key::DELIMITER) {
            return Err(format!("Category must not contain {}", key::DELIMITER).into());
        }
        Ok(Self(s.to_string()))
    }
}

impl TaskFile {
    pub fn create(
        context: &AppContext,
        title: String,
        category: Option<String>,
        description: String,
    ) -> Result<TaskFile, Box<dyn Error>> {
        let category: Category = Category::new(&category.unwrap_or(DEFAULT_CATEGORY.to_string()))?;

        let task_file = TaskFile {
            header: TaskFileHeader {
                key: Key::next_key(context, &category)?,
                category,
                title,
                description,
                status: TaskStatus::Draft,
            },
            body: "".to_string(),
        };
        Ok(task_file)
    }

    pub fn from_data(data: String) -> Result<TaskFile, Box<dyn Error + 'static>> {
        let (header, body) = frontmatter::from_frontmatter_markdown::<TaskFileHeader>(&data)?;
        let result = TaskFile { header, body };
        Ok(result)
    }

    pub fn to_data(&self) -> Result<String, Box<dyn Error>> {
        Ok(tools::frontmatter::to_frontmatter_markdown(
            &self.header,
            &self.body,
        )?)
    }

    pub fn to_one_liner(&self) -> String {
        format!(
            "[{}] {} ({}): {}",
            self.header.key, self.header.title, self.header.status, self.header.description
        )
    }

    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn title(&self) -> std::string::String {
        self.header.title.clone()
    }
}

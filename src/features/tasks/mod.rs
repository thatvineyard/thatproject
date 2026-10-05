mod key;
mod slug;
mod store;
pub mod taskfile;

use std::error::Error;

use crate::{
    app_context::AppContext,
    features::tasks::{
        self,
        key::Key,
        taskfile::{Category, TaskFile, TaskFileHeader, TaskStatus},
    },
    reference::{Reference, ReferenceType},
};

pub fn create_task(
    context: &AppContext,
    title: String,
    category: Option<String>,
    description: String,
) -> Result<TaskFile, Box<dyn Error>> {
    let task_file = taskfile::TaskFile::create(context, title, category, description)?;
    tasks::store::store(context, &task_file)?;
    Ok(task_file)
}

pub fn set_title(context: &AppContext, key: String, title: String) -> Result<(), Box<dyn Error>> {
    let key: Key = key.parse()?;
    tasks::store::update(context, key.into(), |task: TaskFile| {
        Ok(TaskFile {
            header: TaskFileHeader {
                title,
                ..task.header
            },
            ..task
        })
    })
}

pub fn set_category(
    context: &AppContext,
    key: String,
    category: String,
) -> Result<(), Box<dyn Error>> {
    let key: Key = key.parse()?;
    let category = Category::new(category.as_str())?;
    tasks::store::update(context, key.into(), |task: TaskFile| {
        Ok(TaskFile {
            header: TaskFileHeader {
                key: Key::next_key(context, &category)?,
                category,
                ..task.header
            },
            ..task
        })
    })
}

pub fn set_description(
    context: &AppContext,
    key: String,
    description: String,
) -> Result<(), Box<dyn Error>> {
    let key: Key = key.parse()?;
    tasks::store::update(context, key.into(), |task: TaskFile| {
        Ok(TaskFile {
            header: TaskFileHeader {
                description,
                ..task.header
            },
            ..task
        })
    })
}

pub fn set_body(context: &AppContext, key: String, body: String) -> Result<(), Box<dyn Error>> {
    let key: Key = key.parse()?;
    tasks::store::update(context, key.into(), |task| Ok(TaskFile { body, ..task }))
}

pub fn set_status(
    context: &AppContext,
    key: String,
    status: TaskStatus,
) -> Result<(), Box<dyn Error>> {
    let key: Key = key.parse()?;
    tasks::store::update(context, key.into(), |task: TaskFile| {
        Ok(TaskFile {
            header: TaskFileHeader {
                status,
                ..task.header
            },
            ..task
        })
    })
}

pub fn add_reference(
    context: &AppContext,
    key: String,
    reference: String,
    reference_type: ReferenceType,
    note: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let key: Key = key.parse()?;
    let reference = Reference::new(reference, reference_type, note);
    tasks::store::update(context, key.into(), |task: TaskFile| {
        let mut references = task.header.references;
        references.push(reference);
        Ok(TaskFile {
            header: TaskFileHeader {
                references,
                ..task.header
            },
            ..task
        })
    })
}

pub struct ListFilters {
    pub status: Option<TaskStatus>,
}

pub fn list(context: &AppContext, filters: ListFilters) -> Result<(), Box<dyn Error>> {
    let tasks = tasks::store::list(context)?;

    if tasks.is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    let filtered_tasks = tasks
        .iter()
        .filter(|t| filters.status.map_or(true, |s| t.header.status == s));

    for task in filtered_tasks {
        println!("{}", task.to_one_liner());
    }

    Ok(())
}

pub fn read(context: &AppContext, key: String) -> Result<TaskFile, Box<dyn Error>> {
    let key: Key = key.parse()?;
    Ok(store::load(context, key.into())?)
}

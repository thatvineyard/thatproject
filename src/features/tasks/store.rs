use std::{error::Error, fs};

use crate::{
    app_context::AppContext, features::tasks::{slug::slugify, taskfile::TaskFile},
};

const FILE_EXT: &str = ".md";

pub fn store(context: &AppContext, task_file: &TaskFile) -> Result<(), Box<dyn Error>> {
    let data = task_file.to_data()?;
    fs::create_dir_all(context.get_taskfile_dir()?)?;
    fs::write(&file_path(context, &task_file.name())?, data)?;

    Ok(())
}

pub fn load(context: &AppContext, name: String) -> Result<TaskFile, Box<dyn Error>> {
    let path = file_path(context, &name)?;
    let data = fs::read_to_string(path)?;
    let result = TaskFile::from_data(data)?;
    Ok(result)
}

fn file_path(context: &AppContext, name: &String) -> Result<String, Box<dyn Error>> {
    let slug = slugify(name);
    let dir = context.get_taskfile_dir()?;
    Ok(format!("{}/{}{}", dir, slug, FILE_EXT))
}

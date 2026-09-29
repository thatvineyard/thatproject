use std::{error::Error, fs};

use crate::{
    app_context::AppContext,
    features::tasks::{slug::slugify, taskfile::TaskFile},
};

const FILE_EXT: &str = ".md";

pub fn store(context: &AppContext, task_file: &TaskFile) -> Result<(), Box<dyn Error>> {
    let data = task_file.to_data()?;
    fs::create_dir_all(context.get_taskfile_dir()?)?;
    fs::write(&file_path(context, &task_file.name())?, data)?;

    Ok(())
}

pub fn update(
    context: &AppContext,
    name: String,
    update_function: impl FnOnce(TaskFile) -> TaskFile,
) -> Result<(), Box<dyn Error>> {
    let task_file = load(context, name)?;
    let original_file_name = file_path(context, &task_file.name())?;

    let updated_task_file = update_function(task_file);

    let updated_file_name = file_path(context, &updated_task_file.name())?;

    let data = updated_task_file.to_data()?;
    fs::write(updated_file_name.clone(), data)?;
    if updated_file_name != original_file_name {
        fs::remove_file(original_file_name)?;
    }

    Ok(())
}

// pub fn remove(context: &AppContext, task_file: &TaskFile) -> Result<(), Box<dyn Error>> {
//     fs::remove_file(&file_path(context, &task_file.name())?)?;

//     Ok(())
// }

pub fn load(context: &AppContext, name: String) -> Result<TaskFile, Box<dyn Error>> {
    let path = file_path(context, &name)?;
    let data = fs::read_to_string(path)?;
    let result = TaskFile::from_data(data)?;
    Ok(result)
}

pub fn list(context: &AppContext) -> Result<Vec<TaskFile>, Box<dyn Error>> {
    let dir = context.get_taskfile_dir()?;

    let tasks = fs::read_dir(dir)?
        .map(|entry| -> Result<Option<TaskFile>, Box<dyn Error>> {
            let path = entry?.path();

            if !path.is_file() || !path.extension().is_some_and(|ext| ext != FILE_EXT) {
                return Ok(None);
            }
            let data = fs::read_to_string(path)?;
            let result = TaskFile::from_data(data);
            match result {
                Err(_) => Ok(None),
                Ok(result) => Ok(Some(result)),
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(tasks)
}

fn file_path(context: &AppContext, name: &String) -> Result<String, Box<dyn Error>> {
    let slug = slugify(name);
    let dir = context.get_taskfile_dir()?;
    Ok(format!("{}/{}{}", dir, slug, FILE_EXT))
}

use std::{error::Error, fs, path::PathBuf};

use crate::{
    app_context::AppContext,
    features::tasks::{key::Key, slug, taskfile::TaskFile},
};

const FILE_EXT: &str = "md";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIdentifier(String);

impl FileIdentifier {
    fn as_filename(&self) -> PathBuf {
        PathBuf::from(&self.0).with_extension(FILE_EXT)
    }
}

impl From<&TaskFile> for FileIdentifier {
    fn from(task_file: &TaskFile) -> Self {
        task_file.header.key.clone().into()
    }
}

impl From<Key> for FileIdentifier {
    fn from(key: Key) -> Self {
        Self(slug::slugify(key.to_string().as_str()))
    }
}

pub fn store(context: &AppContext, task_file: &TaskFile) -> Result<(), Box<dyn Error>> {
    validate_no_file_exists(context, task_file.into())?;

    let data = task_file.to_data()?;
    fs::create_dir_all(context.get_taskfile_dir()?)?;
    fs::write(&file_path(context, task_file.into())?, data)?;

    Ok(())
}

pub fn update(
    context: &AppContext,
    file: FileIdentifier,
    update_function: impl FnOnce(TaskFile) -> Result<TaskFile, Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let task_file = load(context, file)?;
    let original_file: FileIdentifier = (&task_file).into();

    let updated_task_file = update_function(task_file)?;
    let update_file: FileIdentifier = (&updated_task_file).into();

    let should_move_file = original_file != update_file.clone();
    if should_move_file {
        println!(
            "Renaming file from {} to {}",
            original_file.as_filename().display(),
            update_file.as_filename().display()
        );
        validate_no_file_exists(context, update_file.clone())?;
    }

    let data = updated_task_file.to_data()?;
    fs::write(file_path(context, update_file.clone())?, data)?;

    if should_move_file {
        fs::remove_file(file_path(context, original_file)?)?;
    }

    Ok(())
}

fn validate_no_file_exists(
    context: &AppContext,
    file: FileIdentifier,
) -> Result<(), Box<dyn Error>> {
    if file_exists(context, file)? {
        return Err("File already exists".into());
    }
    Ok(())
}

fn file_exists(context: &AppContext, file: FileIdentifier) -> Result<bool, Box<dyn Error>> {
    let path = file_path(context, file)?;
    Ok(fs::exists(path)?)
}

pub fn load(context: &AppContext, file: FileIdentifier) -> Result<TaskFile, Box<dyn Error>> {
    let path = file_path(context, file)?;
    let data = fs::read_to_string(path)?;
    let result = TaskFile::from_data(data)?;
    Ok(result)
}

pub fn list(context: &AppContext) -> Result<Vec<TaskFile>, Box<dyn Error>> {
    let dir = context.get_taskfile_dir()?;

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };

    let tasks = entries
        .map(|entry| -> Result<Option<TaskFile>, Box<dyn Error>> {
            let path = entry?.path();

            if !path.is_file() || !path.extension().is_some_and(|ext| ext == FILE_EXT) {
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

fn file_path(context: &AppContext, file: FileIdentifier) -> Result<PathBuf, Box<dyn Error>> {
    let dir = context.get_taskfile_dir()?;
    Ok(dir.join(file.as_filename()))
}

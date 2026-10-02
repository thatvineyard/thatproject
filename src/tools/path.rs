use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub fn relativize(base: &Path, target: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let base = base.canonicalize()?;
    let target = target.canonicalize()?;
    let relative = target
        .strip_prefix(&base)
        .map_err(|_| format!("{} is not inside {}", target.display(), base.display()))?;

    Ok(relative.to_path_buf())
}

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::features::manifest::manifest_error::ManifestError;
use crate::features::manifest::manifest_file::ManifestFile;
use crate::features::subproject::subproject::Subproject;
use crate::reference::Reference;

#[derive(Debug)]
pub struct Manifest {
    pub thatproject_dir: PathBuf,
    file: ManifestFile,
}

impl Manifest {
    pub fn create(
        thatproject_dir: &Path,
        name: String,
        description: String,
        task_dir: String,
    ) -> Result<Self, ManifestError> {
        let manifest_file = ManifestFile {
            name,
            description,
            subprojects: Vec::new(),
            references: Vec::new(),
            task_dir,
        };

        let manifest = Manifest {
            thatproject_dir: thatproject_dir.to_path_buf(),
            file: manifest_file,
        };

        Ok(manifest)
    }

    pub fn load(thatproject_dir: &Path) -> Result<Manifest, ManifestError> {
        let manifest_filepath = ManifestFile::path_in(thatproject_dir);
        let content = fs::read_to_string(&manifest_filepath).map_err(|e| match e.kind() {
            ErrorKind::NotFound => ManifestError::NotFound(manifest_filepath.clone()),
            _ => ManifestError::Io(e),
        })?;
        let manifest_file: ManifestFile = content.parse()?;

        let manifest = Manifest {
            thatproject_dir: thatproject_dir.to_path_buf(),
            file: manifest_file,
        };

        Ok(manifest)
    }

    pub fn store(&self) -> Result<(), ManifestError> {
        let manifest_filepath = ManifestFile::path_in(&self.thatproject_dir);
        fs::create_dir_all(&self.thatproject_dir)?;
        fs::write(manifest_filepath, self.file.to_string()?)?;

        Ok(())
    }

    pub fn add_reference(&mut self, reference: Reference) -> Result<(), ManifestError> {
        if !self.file.references.contains(&reference) {
            self.file.references.push(reference);
        }

        Ok(())
    }

    pub fn add_subproject(&mut self, subproject: Subproject) -> Result<(), ManifestError> {
        if !self.file.subprojects.contains(&subproject) {
            self.file.subprojects.push(subproject);
        }

        Ok(())
    }

    pub fn get_taskfile_dir(&self) -> PathBuf {
        self.thatproject_dir.join(self.file.task_dir.clone())
    }

    pub fn to_string(&self, agent_mode: bool) -> Result<String, ManifestError> {
        let data = match agent_mode {
            true => serde_json::to_string(&self.file)?,
            false => serde_yaml::to_string(&self.file)?,
        };

        Ok(data)
    }
}

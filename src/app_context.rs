use std::{error::Error, path::PathBuf};

use crate::{
    config,
    features::manifest::{manifest::Manifest, manifest_error::ManifestError},
};

pub enum ProjectState {
    Valid(ValidatedManifest),
    Unset,
}

pub struct AppContext {
    pub project: ProjectState,
    pub context_dir: PathBuf,
    pub agent_mode: bool,
}

impl AppContext {
    pub fn require_manifest(&self) -> Result<&Manifest, Box<dyn Error>> {
        let ProjectState::Valid(validated) = &self.project else {
            return Err("project manifest is not initialized".into());
        };

        Ok(&validated.manifest)
    }

    pub fn require_manifest_mut(&mut self) -> Result<&mut Manifest, Box<dyn Error>> {
        let ProjectState::Valid(validated) = &mut self.project else {
            return Err("project manifest is not initialized".into());
        };

        Ok(&mut validated.manifest)
    }

    pub fn get_taskfile_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(self.require_manifest()?.get_taskfile_dir())
    }

    pub fn get_thatproject_dir(&self) -> PathBuf {
        self.context_dir.join(config::PROJECT_DIR)
    }
}

pub struct ValidatedManifest {
    pub manifest: Manifest,
}

pub fn load(context_dir: Option<PathBuf>, agent_mode: bool) -> Result<AppContext, Box<dyn Error>> {
    let context_dir = context_dir.unwrap_or_else(|| PathBuf::from(config::DEFAULT_CONTEXT_DIR));

    let project = match Manifest::load(&context_dir.join(config::PROJECT_DIR)) {
        Ok(manifest) => ProjectState::Valid(ValidatedManifest { manifest }),
        Err(error) => match error {
            ManifestError::NotFound(_) => ProjectState::Unset,
            _ => return Err(error.into()),
        },
    };

    Ok(AppContext {
        project,
        context_dir,
        agent_mode,
    })
}

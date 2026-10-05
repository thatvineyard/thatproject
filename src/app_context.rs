use std::{error::Error, path::PathBuf};

use crate::{
    config,
    features::{
        manifest::{manifest::Manifest, manifest_error::ManifestError},
        project::Project,
    },
};

pub enum ProjectState {
    Valid(Project),
    Unset,
}

pub struct AppContext {
    pub project: ProjectState,
    pub context_dir: PathBuf,
    pub agent_mode: bool,
}

impl AppContext {
    pub fn require_project(&self) -> Result<&Project, Box<dyn Error>> {
        let ProjectState::Valid(validated) = &self.project else {
            return Err("project is not initialized".into());
        };

        Ok(&validated)
    }

    pub fn require_project_mut(&mut self) -> Result<&mut Project, Box<dyn Error>> {
        let ProjectState::Valid(validated) = &mut self.project else {
            return Err("project manifest is not initialized".into());
        };

        Ok(validated)
    }

    pub fn get_taskfile_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(self.require_project()?.manifest.get_taskfile_dir())
    }

    pub fn get_thatproject_dir(&self) -> PathBuf {
        self.context_dir.join(config::PROJECT_DIR)
    }
}

pub fn load(
    context_dir: Option<PathBuf>,
    subproject: Option<String>,
    agent_mode: bool,
) -> Result<AppContext, Box<dyn Error>> {
    let context_dir = context_dir.unwrap_or_else(|| PathBuf::from(config::DEFAULT_CONTEXT_DIR));

    let project = match Manifest::load(&context_dir.join(config::PROJECT_DIR)) {
        Ok(manifest) => ProjectState::Valid(Project::resolve(manifest)?),
        Err(error) => match error {
            ManifestError::NotFound(_) => ProjectState::Unset,
            _ => return Err(error.into()),
        },
    };

    let context = match subproject {
        Some(subproject_name) => match &project {
            ProjectState::Valid(project) => project
                .manifest
                .try_get_subproject(&subproject_name)?
                .ok_or("Subproject does not exist.")
                .map(|p| (p.0.path.clone(), ProjectState::Valid(p.1)))?,
            ProjectState::Unset => {
                return Err("Project was not valid. Could not select subproject".into());
            }
        },
        None => (context_dir, project),
    };

    Ok(AppContext {
        project: context.1,
        context_dir: context.0,
        agent_mode,
    })
}

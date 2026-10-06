use std::{error::Error, path::PathBuf};

use crate::{
    config,
    features::{
        manifest::{manifest::Manifest, manifest_error::ManifestError},
        project::Project,
    },
};

pub struct ProjectState {
    root_project: Project,
    project_selection: ProjectSelection,
}

impl ProjectState {
    pub fn get_active_project(&self) -> &Project {
        match &self.project_selection {
            ProjectSelection::Root => &self.root_project,
            ProjectSelection::Subproject(project) => project,
        }
    }

    pub fn get_active_project_mut(&mut self) -> &mut Project {
        match &mut self.project_selection {
            ProjectSelection::Root => &mut self.root_project,
            ProjectSelection::Subproject(project) => project,
        }
    }
}

pub struct AppContext {
    calling_dir: PathBuf,
    root_dir: PathBuf,
    pub project_state: Option<ProjectState>,
    pub agent_mode: bool,
}

enum ProjectSelection {
    Root,
    Subproject(Project),
}

impl AppContext {
    fn require_project_state_mut(&mut self) -> Result<&mut ProjectState, Box<dyn Error>> {
        Ok(self
            .project_state
            .as_mut()
            .ok_or("Project is not initialized")?)
    }

    fn require_project_state(&self) -> Result<&ProjectState, Box<dyn Error>> {
        Ok(self
            .project_state
            .as_ref()
            .ok_or("Project is not initialized")?)
    }

    pub fn require_project(&self) -> Result<&Project, Box<dyn Error>> {
        Ok(self.require_project_state()?.get_active_project())
    }

    pub fn require_project_mut(&mut self) -> Result<&mut Project, Box<dyn Error>> {
        Ok(self.require_project_state_mut()?.get_active_project_mut())
    }

    pub fn get_taskfile_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(self.require_project()?.manifest.get_taskfile_dir())
    }

    pub fn get_thatproject_dir(&self) -> PathBuf {
        self.root_dir.join(config::PROJECT_DIR)
    }

    pub fn root_dir(&self) -> PathBuf {
        self.calling_dir.join(&self.root_dir)
    }
}

pub fn load(
    root_dir: Option<PathBuf>,
    subproject: Option<String>,
    agent_mode: bool,
) -> Result<AppContext, Box<dyn Error>> {
    let calling_dir = std::env::current_dir()?;

    let root_dir = root_dir.unwrap_or_else(|| PathBuf::from(config::DEFAULT_CONTEXT_DIR));

    let project_state = match Manifest::load(&root_dir.join(config::PROJECT_DIR)) {
        Ok(manifest) => Some(resolve_project_state(manifest, subproject)?),
        Err(error) => match error {
            ManifestError::NotFound(_) => {
                if subproject.is_some() {
                    return Err("Project was not valid. Cannot parse subproject".into());
                };
                None
            }
            _ => return Err(error.into()),
        },
    };

    Ok(AppContext {
        calling_dir,
        root_dir,
        project_state,
        agent_mode,
    })
}

fn resolve_project_state(
    manifest: Manifest,
    subproject: Option<String>,
) -> Result<ProjectState, Box<dyn Error>> {
    let root_project = Project::resolve(manifest)?;

    let project_selection = match subproject {
        Some(subproject_name) => root_project
            .manifest
            .try_get_subproject(&subproject_name)?
            .ok_or("Subproject does not exist.")
            .map(|p| ProjectSelection::Subproject(p.1))?,
        None => ProjectSelection::Root,
    };

    Ok(ProjectState {
        root_project,
        project_selection,
    })
}

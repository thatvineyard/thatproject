
use crate::manifest::Manifest;

enum ProjectState {
    Valid(ValidatedManifest),
    Unset,
}

pub struct AppContext {
    project: ProjectState,
}

impl AppContext {
    pub fn get_taskfile_dir(&self) -> Result<String, std::io::Error> {
      let ProjectState::Valid(validated) = &self.project else {
        return Err(std::io::Error::new(
          std::io::ErrorKind::NotFound,
            "project manifest is not initialized",
        ));
      };

      Ok(validated.manifest.get_taskfile_dir())
    }
}

struct ValidatedManifest {
  manifest: Manifest,
}

pub fn load() -> Result<AppContext, std::io::Error> {
  let manifest = match Manifest::load() {
    Ok(manifest) => manifest,

    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
      return Ok(AppContext { project: ProjectState::Unset });
    }

    Err(error) => return Err(error),
  };

  Ok(AppContext { 
    project: ProjectState::Valid(ValidatedManifest { manifest }) 
  })
}
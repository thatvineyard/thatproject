use std::{collections::BTreeMap, error::Error};

use serde::Serialize;

use crate::{
    app_context::AppContext, features::manifest::manifest::Manifest, reference::Reference,
};

pub struct Project {
    pub manifest: Manifest,
}

impl Project {
    pub fn resolve(manifest: Manifest) -> Result<Self, Box<dyn Error>> {
        let project = Project { manifest };

        Ok(project)
    }

    pub fn to_summary(&self, context: &AppContext) -> Result<String, Box<dyn Error>> {
        let output = FullProjectSummary::try_from(self)?.to_string(context.agent_mode)?;

        Ok(output)
    }
}

#[derive(Serialize)]
pub struct FullProjectSummary {
    pub name: String,
    pub description: String,
    pub references: Vec<Reference>,
    pub subprojects: BTreeMap<String, SlimProjectSummary>,
}

#[derive(Serialize)]
pub struct SlimProjectSummary {
    pub name: String,
    pub description: String,
}

impl From<&Project> for SlimProjectSummary {
    fn from(p: &Project) -> Self {
        Self {
            name: p.manifest.name().to_string(),
            description: p.manifest.description().to_string(),
        }
    }
}

impl TryFrom<&Project> for FullProjectSummary {
    type Error = Box<dyn Error>;

    fn try_from(p: &Project) -> Result<Self, Self::Error> {
        let subprojects: BTreeMap<String, SlimProjectSummary> = p
            .manifest
            .resolve_subprojects()?
            .iter()
            .map(|(r, sp)| {
                (
                    r.alias
                        .clone()
                        .unwrap_or_else(|| sp.manifest.name().to_owned()),
                    SlimProjectSummary::from(sp),
                )
            })
            .collect();

        Ok(Self {
            name: p.manifest.name().to_string(),
            description: p.manifest.description().to_string(),
            references: p.manifest.references().clone(),
            subprojects,
        })
    }
}

impl FullProjectSummary {
    fn to_string(&self, agent_mode: bool) -> Result<String, Box<dyn Error>> {
        let data = match agent_mode {
            true => serde_json::to_string(&self)?,
            false => serde_yaml::to_string(&self)?,
        };

        Ok(data)
    }
}

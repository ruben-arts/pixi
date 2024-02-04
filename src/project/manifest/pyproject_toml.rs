use crate::project::manifest::environment::TomlEnvironmentMapOrSeq;
use crate::project::manifest::{
    Activation, Environment, Feature, ProjectManifest, ProjectMetadata, PyPiRequirement,
    SystemRequirements, Target, TargetSelector, Targets,
};
use crate::utils::spanned::PixiSpanned;
use crate::FeatureName::Default;
use crate::{EnvironmentName, FeatureName, SpecType, Task};
use indexmap::IndexMap;
use itertools::Itertools;
use pep508_rs::VersionOrUrl::VersionSpecifier;
use rattler_conda_types::{NamelessMatchSpec, PackageName, Version};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, Map, PickFirst};
use std::collections::HashMap;
use std::str::FromStr;
use toml_edit::TomlError;

#[derive(Deserialize)]
pub struct PyProjectToml {
    #[serde(flatten)]
    inner: pyproject_toml::PyProjectToml,
    tool: Option<Tool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Tool {
    pixi: Option<ToolPixi>,
}

#[serde_as]
#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ToolPixi {
    #[serde(default)]
    project: ProjectMetadata,
    #[serde(default)]
    system_requirements: SystemRequirements,
    #[serde(default)]
    target: IndexMap<PixiSpanned<TargetSelector>, Target>,

    #[serde(default)]
    #[serde_as(as = "IndexMap<_, PickFirst<(DisplayFromStr, _)>>")]
    dependencies: IndexMap<PackageName, NamelessMatchSpec>,

    #[serde(default)]
    #[serde_as(as = "Option<IndexMap<_, PickFirst<(DisplayFromStr, _)>>>")]
    host_dependencies: Option<IndexMap<PackageName, NamelessMatchSpec>>,

    #[serde(default)]
    #[serde_as(as = "Option<IndexMap<_, PickFirst<(DisplayFromStr, _)>>>")]
    build_dependencies: Option<IndexMap<PackageName, NamelessMatchSpec>>,

    #[serde(default)]
    pypi_dependencies: Option<IndexMap<rip::types::PackageName, PyPiRequirement>>,

    /// Additional information to activate an environment.
    #[serde(default)]
    activation: Option<Activation>,

    /// Target specific tasks to run in the environment
    #[serde(default)]
    tasks: HashMap<String, Task>,

    /// The features defined in the project.
    #[serde(default)]
    feature: IndexMap<FeatureName, Feature>,

    /// The environments the project can create.
    #[serde(default)]
    #[serde_as(as = "Map<_, _>")]
    environments: Vec<(EnvironmentName, TomlEnvironmentMapOrSeq)>,
}

impl std::ops::Deref for PyProjectToml {
    type Target = pyproject_toml::PyProjectToml;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PyProjectToml {
    pub fn from_toml_str(source: &str) -> Result<Self, TomlError> {
        toml_edit::de::from_str(source).map_err(TomlError::from)
    }
}

impl ProjectManifest {
    pub fn from_pyproject_toml(pyproject_toml: PyProjectToml) -> miette::Result<Self> {
        let pixi = pyproject_toml
            .tool
            .as_ref()
            .and_then(|tool| tool.pixi.clone());
        if let Some(pixi) = pixi {
            let mut dependencies = HashMap::from_iter([(SpecType::Run, pixi.dependencies)]);

            if let Some(host_deps) = pixi.host_dependencies {
                dependencies.insert(SpecType::Host, host_deps);
            }
            if let Some(build_deps) = pixi.build_dependencies {
                dependencies.insert(SpecType::Build, build_deps);
            }
            let mut pypi_map = IndexMap::new();
            let _ = pyproject_toml
                .project
                .clone()
                .unwrap()
                .dependencies
                .unwrap()
                .into_iter()
                .map(|requirement| {
                    let version = match requirement.version_or_url {
                        Some(VersionSpecifier(version)) => Some(version),
                        _ => None,
                    };
                    pypi_map.insert(
                        rip::types::PackageName::from_str(requirement.name.as_str()).ok()?,
                        PyPiRequirement {
                            version,
                            extras: requirement.extras,
                        },
                    )
                })
                .collect_vec();

            let default_target = Target {
                dependencies,
                pypi_dependencies: Some(pypi_map),
                activation: pixi.activation,
                tasks: pixi.tasks,
            };

            // Construct a default feature
            let default_feature = Feature {
                name: FeatureName::Default,

                // The default feature does not overwrite the platforms or channels from the project
                // metadata.
                platforms: None,
                channels: None,

                system_requirements: pixi.system_requirements,

                // Combine the default target with all user specified targets
                targets: Targets::from_default_and_user_defined(default_target, pixi.target),
            };

            // Construct a default environment
            let default_environment = Environment {
                name: EnvironmentName::Default,
                features: Vec::new(),
                features_source_loc: None,
                solve_group: None,
            };

            // Construct the features including the default feature
            let features: IndexMap<FeatureName, Feature> =
                IndexMap::from_iter([(FeatureName::Default, default_feature)]);
            let named_features = pixi
                .feature
                .into_iter()
                .map(|(name, mut feature)| {
                    feature.name = name.clone();
                    (name, feature)
                })
                .collect::<IndexMap<FeatureName, Feature>>();
            let features = features.into_iter().chain(named_features).collect();

            // Construct the environments including the default environment
            let environments: IndexMap<EnvironmentName, Environment> =
                IndexMap::from_iter([(EnvironmentName::Default, default_environment)]);
            let named_environments = pixi
                .environments
                .into_iter()
                .map(|(name, t_env)| {
                    let env = t_env.into_environment(name.clone());
                    (name, env)
                })
                .collect::<IndexMap<EnvironmentName, Environment>>();
            let environments = environments.into_iter().chain(named_environments).collect();

            let project_toml = pyproject_toml.inner.project.unwrap();
            let project = ProjectMetadata {
                name: project_toml.name,
                version: pixi.project.version,
                description: None,
                authors: Vec::new(),
                channels: pixi.project.channels,
                platforms: pixi.project.platforms,

                license: pixi.project.license,
                license_file: pixi.project.license_file,
                readme: pixi.project.readme,
                homepage: pixi.project.homepage,
                repository: pixi.project.repository,
                documentation: pixi.project.documentation,
            };
            Ok(Self {
                project,
                features,
                environments,
            })
        } else {
            Err(miette::miette!(
                "No [tool.pixi] section found in pyproject.toml"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyproject_toml() {
        let content = r#"
[project]
name = "pixi"
version = "0.1.0"
description = "A Python package"

dependencies = [
    "httpx",
    "pydantic",
    "typer",
    "toml ==0.5.8"
]

[tool.pixi]
dependencies = { "numpy" = "*" , python = "*"}
        "#;
        let pyproject_toml = PyProjectToml::from_toml_str(content).unwrap();
        assert_eq!(pyproject_toml.inner.project.clone().unwrap().name, "pixi");
        assert_eq!(
            pyproject_toml
                .inner
                .project
                .clone()
                .unwrap()
                .version
                .unwrap()
                .to_string(),
            "0.1.0".to_string()
        );
        let manifest = ProjectManifest::from_pyproject_toml(pyproject_toml).unwrap();
    }
}

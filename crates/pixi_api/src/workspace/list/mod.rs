use std::collections::HashSet;

use pixi_core::{Workspace, lock_file::UpdateLockFileOptions};
use pixi_manifest::{EnvironmentName, FeaturesExt};
use rattler_conda_types::Platform;
use serde::Serialize;
use uv_normalize::PackageName;

use crate::interface::Interface;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum PackageKind {
    Conda,
    Pypi,
}

#[derive(Debug, Clone, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    pub kind: PackageKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub is_explicit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct ListPackagesOptions {
    pub environment: Option<EnvironmentName>,
    pub platform: Option<Platform>,
    pub explicit_only: bool,
}

pub async fn list_packages<I: Interface>(
    _interface: &I,
    workspace: &Workspace,
    options: ListPackagesOptions,
) -> miette::Result<Vec<Package>> {
    let environment = if let Some(env_name) = options.environment {
        workspace
            .environment(&env_name)
            .ok_or_else(|| miette::miette!("unknown environment '{}'", env_name))?
    } else {
        workspace.default_environment()
    };

    // Get or update the lock file
    let lock_file = workspace
        .update_lock_file(UpdateLockFileOptions {
            lock_file_usage: pixi_core::environment::LockFileUsage::Update,
            no_install: true,
            max_concurrent_solves: workspace.config().max_concurrent_solves(),
        })
        .await?
        .0
        .into_lock_file();

    let platform = options
        .platform
        .unwrap_or_else(|| environment.best_platform());

    // Get all the packages in the environment
    let locked_deps = lock_file
        .environment(environment.name().as_str())
        .and_then(|env| env.packages(platform).map(Vec::from_iter))
        .unwrap_or_default();

    // Get the explicit project dependencies (using FeaturesExt trait)
    let mut project_dependency_names: HashSet<String> = environment
        .combined_dependencies(Some(platform))
        .names()
        .map(|p| p.as_source().to_string())
        .collect();

    project_dependency_names.extend(
        environment
            .pypi_dependencies(Some(platform))
            .into_iter()
            .map(|(name, _)| name.as_source().to_string()),
    );

    let mut packages = Vec::new();

    for locked_pkg in locked_deps {
        match locked_pkg {
            rattler_lock::LockedPackageRef::Conda(conda_pkg) => {
                let record = conda_pkg.record();
                let name = record.name.as_source().to_string();
                let is_explicit = project_dependency_names.contains(&name);

                if options.explicit_only && !is_explicit {
                    continue;
                }

                let source = match &conda_pkg {
                    rattler_lock::CondaPackageData::Binary(binary) => {
                        binary.channel.as_ref().map(|c| c.to_string())
                    }
                    rattler_lock::CondaPackageData::Source(source) => {
                        Some(source.location.to_string())
                    }
                };

                packages.push(Package {
                    name,
                    version: record.version.version().to_string(),
                    build: Some(record.build.clone()),
                    size_bytes: record.size,
                    kind: PackageKind::Conda,
                    source,
                    is_explicit,
                    is_editable: None,
                });
            }
            rattler_lock::LockedPackageRef::Pypi(pypi_pkg, _) => {
                let name = pypi_pkg.name.to_string();
                let is_explicit = project_dependency_names.contains(&name);

                if options.explicit_only && !is_explicit {
                    continue;
                }

                let (size_bytes, source) = match &pypi_pkg.location {
                    rattler_lock::UrlOrPath::Url(url) => (None, Some(url.to_string())),
                    rattler_lock::UrlOrPath::Path(path) => (None, Some(path.to_string())),
                };

                packages.push(Package {
                    name,
                    version: pypi_pkg.version.to_string(),
                    build: None,
                    size_bytes,
                    kind: PackageKind::Pypi,
                    source,
                    is_explicit,
                    is_editable: Some(pypi_pkg.editable),
                });
            }
        }
    }

    // Sort by name
    packages.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(packages)
}

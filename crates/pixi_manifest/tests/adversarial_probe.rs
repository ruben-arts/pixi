//! Adversarial probes for PR #6787 (feature platform leak across
//! environments, issue #6770).
//!
//! Uses only public pixi_manifest API so the same file compiles against both
//! the PR head and the merge-base (f14d73d). Every probe prints its observed
//! platform sets with a `PROBE` prefix BEFORE any assertion, so a failing run
//! still reports the actual values.

use std::path::Path;

use pixi_manifest::{
    Feature, FeaturesExt, HasFeaturesIter, HasWorkspaceManifest, WorkspaceManifest,
};

/// Mirrors pixi_core's `Environment`: the named environment's features,
/// chained with the default feature unless `no-default-feature` is set.
struct EnvFeatures<'a> {
    manifest: &'a WorkspaceManifest,
    features: Vec<&'a Feature>,
}

impl<'a> HasWorkspaceManifest<'a> for EnvFeatures<'a> {
    fn workspace_manifest(&self) -> &'a WorkspaceManifest {
        self.manifest
    }
}

impl<'a> HasFeaturesIter<'a> for EnvFeatures<'a> {
    fn features(&self) -> impl DoubleEndedIterator<Item = &'a Feature> + 'a {
        self.features.clone().into_iter()
    }
}

fn parse(source: &str) -> WorkspaceManifest {
    match WorkspaceManifest::from_toml_str_with_base_dir(source, Path::new("")) {
        Ok(manifest) => manifest,
        Err(e) => panic!("manifest failed to parse: {:?}", e.error),
    }
}

fn env_features<'a>(manifest: &'a WorkspaceManifest, name: &str) -> EnvFeatures<'a> {
    let env = manifest
        .environments
        .iter()
        .find(|e| e.name.as_str() == name)
        .unwrap_or_else(|| panic!("environment '{name}' not found"));
    let mut features: Vec<&Feature> = env
        .features
        .iter()
        .map(|fname| {
            manifest
                .feature(fname)
                .unwrap_or_else(|| panic!("feature '{}' not found", fname.as_str()))
        })
        .collect();
    if !env.no_default_feature {
        features.push(manifest.default_feature());
    }
    EnvFeatures { manifest, features }
}

/// Sorted platform *names* of the environment, exactly what
/// `FeaturesExt::platforms` yields.
fn platform_names(manifest: &WorkspaceManifest, env: &str) -> Vec<String> {
    let mut names: Vec<String> = env_features(manifest, env)
        .platforms()
        .into_iter()
        .map(|n| n.as_str().to_string())
        .collect();
    names.sort();
    names
}

/// Sorted conda *subdirs* of the environment, resolved through the workspace
/// platform registry (names on the composition path can be rich, e.g.
/// `linux-64-cuda-12-0`).
fn platform_subdirs(manifest: &WorkspaceManifest, env: &str) -> Vec<String> {
    let mut subdirs: Vec<String> = env_features(manifest, env)
        .platforms()
        .into_iter()
        .map(|name| {
            manifest
                .workspace
                .platforms
                .iter()
                .find(|p| *p.name() == name)
                .map(|p| p.subdir().as_str().to_string())
                .unwrap_or_else(|| format!("<unresolvable name {}>", name.as_str()))
        })
        .collect();
    subdirs.sort();
    subdirs.dedup();
    subdirs
}

fn workspace_platform_names(manifest: &WorkspaceManifest) -> Vec<String> {
    manifest
        .workspace
        .platforms
        .iter()
        .map(|p| p.name().as_str().to_string())
        .collect()
}

/// Every feature's (possibly migration-rewritten) `platforms` list, for
/// diagnosing which mechanism carried a subdir into an environment.
fn feature_platform_lists(manifest: &WorkspaceManifest, env_names: &[&str]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    for env_name in env_names {
        let env = manifest
            .environments
            .iter()
            .find(|e| e.name.as_str() == *env_name)
            .unwrap();
        for fname in &env.features {
            let feature = manifest.feature(fname).unwrap();
            let label = fname.as_str().to_string();
            if seen.contains(&label) {
                continue;
            }
            seen.push(label.clone());
            out.push(format!(
                "{label}={:?}",
                feature.platforms.as_ref().map(|names| {
                    names.iter().map(|n| n.as_str().to_string()).collect::<Vec<_>>()
                })
            ));
        }
    }
    let default = manifest.default_feature();
    out.push(format!(
        "default={:?}",
        default.platforms.as_ref().map(|names| {
            names.iter().map(|n| n.as_str().to_string()).collect::<Vec<_>>()
        })
    ));
    out
}

/// Probe 1 — the PR's own regression scenario, observed at the
/// pixi_manifest level (same manifest as the pixi_core test
/// `test_feature_platform_does_not_leak_across_environments`).
#[test]
fn probe1_pr_regression_no_leak() {
    let manifest = parse(
        r#"
        [workspace]
        name = "repro"
        channels = []
        platforms = ["linux-64"]

        [environments]
        dev = { features = ["dev"], no-default-feature = true }

        [feature.dev]
        platforms = ["linux-64", "osx-arm64"]
        "#,
    );
    let default = platform_names(&manifest, "default");
    let dev = platform_names(&manifest, "dev");
    println!("PROBE probe1 workspace.platforms={:?}", workspace_platform_names(&manifest));
    println!("PROBE probe1 default={default:?} dev={dev:?}");
    assert_eq!(default, vec!["linux-64"], "default env must stay on linux-64");
    assert_eq!(dev, vec!["linux-64", "osx-arm64"], "dev env must span both");
}

/// Probe 2 — SYSREQS BYPASS variant A: same manifest as probe 1 but the dev
/// feature ALSO carries system-requirements whose macos version (14.0) does
/// not collapse to the subdir default (13.0), so migration synthesises a rich
/// `osx-arm64-…` platform. Does the leak come back through it?
#[test]
fn probe2_sysreqs_on_leaking_feature() {
    let manifest = parse(
        r#"
        [workspace]
        name = "repro"
        channels = []
        platforms = ["linux-64"]

        [environments]
        dev = { features = ["dev"], no-default-feature = true }

        [feature.dev]
        platforms = ["linux-64", "osx-arm64"]

        [feature.dev.system-requirements]
        macos = "14.0"
        "#,
    );
    let default_names = platform_names(&manifest, "default");
    let default_subdirs = platform_subdirs(&manifest, "default");
    let dev_names = platform_names(&manifest, "dev");
    println!("PROBE probe2 workspace.platforms={:?}", workspace_platform_names(&manifest));
    println!(
        "PROBE probe2 feature.platforms {:?}",
        feature_platform_lists(&manifest, &["dev"])
    );
    println!(
        "PROBE probe2 default_names={default_names:?} default_subdirs={default_subdirs:?} dev_names={dev_names:?}"
    );
    assert_eq!(
        default_subdirs,
        vec!["linux-64"],
        "default env must not reach osx-arm64 even when the dev feature carries system-requirements"
    );
}

/// Probe 3 — SYSREQS BYPASS variant B: the system-requirements live on a
/// DIFFERENT feature ('cuda', part of the default environment, no `platforms`
/// key of its own) while the dev feature adds osx-arm64 without sysreqs.
#[test]
fn probe3_sysreqs_on_default_env_feature() {
    let manifest = parse(
        r#"
        [workspace]
        name = "repro"
        channels = []
        platforms = ["linux-64"]

        [environments]
        default = ["cuda"]
        dev = { features = ["dev"], no-default-feature = true }

        [feature.cuda.system-requirements]
        cuda = "12.0"

        [feature.dev]
        platforms = ["linux-64", "osx-arm64"]
        "#,
    );
    let default_names = platform_names(&manifest, "default");
    let default_subdirs = platform_subdirs(&manifest, "default");
    let dev_names = platform_names(&manifest, "dev");
    println!("PROBE probe3 workspace.platforms={:?}", workspace_platform_names(&manifest));
    println!(
        "PROBE probe3 feature.platforms {:?}",
        feature_platform_lists(&manifest, &["default", "dev"])
    );
    println!(
        "PROBE probe3 default_names={default_names:?} default_subdirs={default_subdirs:?} dev_names={dev_names:?}"
    );
    assert_eq!(
        default_subdirs,
        vec!["linux-64"],
        "default env (with cuda sysreqs feature) must not reach osx-arm64"
    );
}

/// Probe 4 — mixed-env semantics: env 'mixed' includes the default feature
/// plus dev (which declares an extra subdir). NO right/wrong assertion — this
/// records what the code yields (union vs intersection semantics).
#[test]
fn probe4_mixed_env_semantics() {
    let manifest = parse(
        r#"
        [workspace]
        name = "repro"
        channels = []
        platforms = ["linux-64"]

        [environments]
        mixed = { features = ["dev"] }

        [feature.dev]
        platforms = ["linux-64", "osx-arm64"]
        "#,
    );
    let default_names = platform_names(&manifest, "default");
    let mixed_names = platform_names(&manifest, "mixed");
    let mixed_subdirs = platform_subdirs(&manifest, "mixed");
    println!("PROBE probe4 workspace.platforms={:?}", workspace_platform_names(&manifest));
    println!(
        "PROBE probe4 default={default_names:?} mixed_names={mixed_names:?} mixed_subdirs={mixed_subdirs:?}"
    );
    // Record-only: no assertion on mixed. Default env should still be clean;
    // assert it last so the mixed value always prints.
    assert_eq!(default_names, vec!["linux-64"]);
}

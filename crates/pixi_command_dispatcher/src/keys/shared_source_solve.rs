//! ROS shared source build/host solve groups.
//!
//! This module takes the provisional source records discovered by the normal
//! recursive resolver, performs one shared build solve and one shared host solve
//! for ROS outputs that opted in, then projects each member's own closure back
//! into its `SourceRecord`.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    sync::Arc,
};

use itertools::Either;
use pixi_build_types::procedures::conda_outputs::CondaOutput;
use pixi_compute_engine::ComputeCtx;
use pixi_record::{PinnedSourceSpec, PixiRecord, SourceRecord};
use pixi_spec::{BinarySpec, BuildDependencyMode, PixiSpec, SourceAnchor, SourceLocationSpec};
use pixi_spec_containers::DependencyMap;
use pixi_variant::VariantValue;
use rattler_conda_types::{
    MatchSpec, PackageName, PackageNameMatcher, ParseMatchSpecOptions, Platform,
};
use rattler_solve::SolveStrategy;

use crate::{
    EnvironmentRef, InstalledSourceHints, PtrArc, SolvePixiEnvironmentError, SourceMetadata,
    build::{Dependencies, PinnedSourceCodeLocation},
    compute_data::HasGateway,
    keys::{
        resolve_source_record::finalize_source_record,
        solve_conda::{SolveCondaKey, SolveCondaKeyError, SolveCondaSpec},
        source_metadata::{SourceMetadataKey, SourceMetadataSpec},
    },
};

#[derive(Clone)]
struct Member {
    id: MemberId,
    source: PinnedSourceCodeLocation,
    output: CondaOutput,
    build_dependencies: Dependencies,
    host_dependencies: Option<Dependencies>,
    build_run_exports: Vec<(PackageName, crate::build::PixiRunExports)>,
    build_records: Vec<PixiRecord>,
    host_records: Vec<PixiRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct MemberId {
    name: PackageName,
    manifest: String,
    build: Option<String>,
    variant: BTreeMap<String, VariantValue>,
}

impl Ord for MemberId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name
            .as_normalized()
            .cmp(other.name.as_normalized())
            .then_with(|| self.manifest.cmp(&other.manifest))
            .then_with(|| self.build.cmp(&other.build))
            .then_with(|| self.variant.cmp(&other.variant))
    }
}

impl PartialOrd for MemberId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(super) async fn apply_shared_source_solves(
    ctx: &mut ComputeCtx,
    provisional: Vec<Arc<SourceRecord>>,
    env_ref: &EnvironmentRef,
    shared_workspace_dependencies: &Arc<DependencyMap<PackageName, PixiSpec>>,
    preferred_build_source: &Arc<BTreeMap<PackageName, PinnedSourceSpec>>,
    installed_source_hints: &PtrArc<InstalledSourceHints>,
    installed: &[pixi_record::UnresolvedPixiRecord],
    inline_packages: &BTreeMap<PackageName, crate::InlinePackage>,
    env_spec: &crate::EnvironmentSpec,
    solve_strategy: SolveStrategy,
    exclude_newer: pixi_spec::ResolvedExcludeNewer,
) -> Result<Vec<Arc<SourceRecord>>, SolvePixiEnvironmentError> {
    // Group orchestration belongs to the consuming environment. Derived
    // build/host solves remain package-local during the bootstrap pass.
    if matches!(env_ref, EnvironmentRef::Derived { .. }) {
        return Ok(provisional);
    }

    let original = provisional.clone();
    let provisional = collect_source_records(provisional);
    let mut members = discover_members(
        ctx,
        &provisional,
        env_ref,
        shared_workspace_dependencies,
        preferred_build_source,
        installed_source_hints,
        inline_packages,
    )
    .await?;
    if members.is_empty() {
        return Ok(original);
    }

    if env_spec.build_environment.build_platform != env_spec.build_environment.host_platform {
        return Err(SolvePixiEnvironmentError::UnsupportedSharedSourceSolve(
            "ROS shared build-dependency-mode currently supports native builds only; use build-dependency-mode = \"isolated\" for ROS source packages when cross-compiling".to_string(),
        ));
    }

    let installed: Vec<PixiRecord> = installed
        .iter()
        .filter_map(|record| record.clone().try_into_resolved().ok())
        .collect();
    let mut current = provisional;
    let mut seen_states = HashSet::new();
    for _ in 0..6 {
        let source_repodata = source_repodata_from_records(current.clone());

        let (build_deps, build_constraints) =
            merge_member_requirements(members.iter().map(|m| &m.build_dependencies));
        let build_pool = solve_group(
            ctx,
            env_spec,
            build_deps,
            build_constraints,
            source_repodata.clone(),
            installed.clone(),
            env_spec.build_environment.build_platform,
            env_spec.build_environment.build_virtual_packages.clone(),
            solve_strategy,
            exclude_newer.clone(),
        )
        .await?;
        for member in members.iter_mut() {
            member.build_records =
                project_dependency_closure(&build_pool, &member.build_dependencies);
            let gateway = ctx.global_data().gateway().clone();
            let mut records_for_exports = member.build_records.clone();
            member.build_run_exports = member
                .build_dependencies
                .extract_run_exports(
                    &mut records_for_exports,
                    &member.output.ignore_run_exports,
                    &gateway,
                    None,
                )
                .await
                .map_err(|err| SolvePixiEnvironmentError::ResolveSourcePackage {
                    name: member.id.name.clone(),
                    source: Box::new(SourceLocationSpec::from(
                        member.source.manifest_source().clone(),
                    )),
                    error: Box::new(crate::SourceRecordError::RunExportsExtraction(
                        "build".to_string(),
                        Arc::new(err),
                    )),
                })?;

            let compatibility_map: HashMap<_, _> = member
                .build_records
                .iter()
                .map(|record| (record.package_record().name.clone(), record))
                .collect();
            let source_anchor = SourceAnchor::from(SourceLocationSpec::from(
                member.source.manifest_source().clone(),
            ));
            let host = member
                .output
                .host_dependencies
                .as_ref()
                .map(|deps| Dependencies::new(deps, Some(source_anchor), &compatibility_map))
                .transpose()
                .map_err(crate::SourceRecordError::from)
                .map_err(|error| SolvePixiEnvironmentError::ResolveSourcePackage {
                    name: member.id.name.clone(),
                    source: Box::new(SourceLocationSpec::from(
                        member.source.manifest_source().clone(),
                    )),
                    error: Box::new(error),
                })?
                .unwrap_or_default()
                .extend_with_shared_workspace_dependencies(shared_workspace_dependencies)
                .extend_with_run_exports_from_build(&member.build_run_exports);
            member.host_dependencies = Some(host);
        }

        let (host_deps, host_constraints) = merge_member_requirements(
            members
                .iter()
                .map(|m| m.host_dependencies.as_ref().expect("host prepared")),
        );
        let host_pool = solve_group(
            ctx,
            env_spec,
            host_deps,
            host_constraints,
            source_repodata,
            installed.clone(),
            env_spec.build_environment.host_platform,
            env_spec.build_environment.host_virtual_packages.clone(),
            solve_strategy,
            exclude_newer.clone(),
        )
        .await?;
        for member in members.iter_mut() {
            member.host_records = project_dependency_closure(
                &host_pool,
                member.host_dependencies.as_ref().expect("host prepared"),
            );
        }

        let mut replacements = HashMap::new();
        for member in members.iter() {
            let record = finalize_source_record(
                ctx,
                &member.source,
                &member.output,
                member.build_dependencies.clone(),
                member.build_records.clone(),
                member.build_run_exports.clone(),
                member.host_dependencies.clone().expect("host prepared"),
                member.host_records.clone(),
                inline_packages
                    .get(&member.id.name)
                    .map(|inline| inline.content_hash),
            )
            .await
            .map_err(|error| SolvePixiEnvironmentError::ResolveSourcePackage {
                name: member.id.name.clone(),
                source: Box::new(SourceLocationSpec::from(
                    member.source.manifest_source().clone(),
                )),
                error: Box::new(error),
            })?;
            replacements.insert(member.id.clone(), record);
        }
        let next: Vec<_> = current
            .iter()
            .map(|record| {
                let id = member_id_for_record(record);
                replacements
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| Arc::clone(record))
            })
            .collect();
        let state = state_fingerprint(&next);
        if state == state_fingerprint(&current) {
            return Ok(next);
        }
        if !seen_states.insert(state) {
            return Err(SolvePixiEnvironmentError::UnsupportedSharedSourceSolve(
                "ROS shared source solve did not converge (repeated state)".to_string(),
            ));
        }
        current = next;
        members = discover_members(
            ctx,
            &current,
            env_ref,
            shared_workspace_dependencies,
            preferred_build_source,
            installed_source_hints,
            inline_packages,
        )
        .await?;
    }

    Err(SolvePixiEnvironmentError::UnsupportedSharedSourceSolve(
        "ROS shared source solve did not converge within 6 iterations".to_string(),
    ))
}

async fn discover_members(
    ctx: &mut ComputeCtx,
    records: &[Arc<SourceRecord>],
    env_ref: &EnvironmentRef,
    shared_workspace_dependencies: &DependencyMap<PackageName, PixiSpec>,
    preferred_build_source: &BTreeMap<PackageName, PinnedSourceSpec>,
    _installed_source_hints: &PtrArc<InstalledSourceHints>,
    inline_packages: &BTreeMap<PackageName, crate::InlinePackage>,
) -> Result<Vec<Member>, SolvePixiEnvironmentError> {
    let mut members = Vec::new();
    for record in records {
        let name = record.package_record().name.clone();
        let source_location = SourceLocationSpec::from(record.manifest_source().clone());
        let own_pin = preferred_build_source.get(&name).cloned();
        let outputs = ctx
            .compute(&SourceMetadataKey::new(SourceMetadataSpec {
                package: name.clone(),
                source_location: source_location.clone(),
                preferred_build_source: own_pin,
                manifest_pin_override: Some(record.manifest_source().clone()),
                env_ref: env_ref.clone(),
                inline: inline_packages.get(&name).cloned(),
            }))
            .await
            .map_err(SolvePixiEnvironmentError::from)?;
        let Some(output) = outputs
            .outputs
            .iter()
            .find(|output| {
                output.metadata.name == name
                    && output.metadata.version == record.package_record().version
                    && output.metadata.build == record.package_record().build
                    && output
                        .metadata
                        .variant
                        .iter()
                        .map(|(k, v)| (k.clone(), VariantValue::from(v.clone())))
                        .collect::<BTreeMap<_, _>>()
                        == record.variants
            })
            .cloned()
        else {
            continue;
        };
        if !output.shared_workspace_dependencies {
            continue;
        }
        validate_isolation_policy(&name, shared_workspace_dependencies)?;
        if is_isolated(&name, shared_workspace_dependencies) {
            continue;
        }
        let source_anchor = SourceAnchor::from(source_location.clone());
        let build_dependencies = output
            .build_dependencies
            .as_ref()
            .map(|deps| Dependencies::new(deps, Some(source_anchor), &HashMap::new()))
            .transpose()
            .map_err(crate::SourceRecordError::from)
            .map_err(|error| SolvePixiEnvironmentError::ResolveSourcePackage {
                name: name.clone(),
                source: Box::new(source_location.clone()),
                error: Box::new(error),
            })?
            .unwrap_or_default()
            .extend_with_shared_workspace_dependencies(shared_workspace_dependencies);
        members.push(Member {
            id: member_id_for_record(record),
            source: outputs.source.clone(),
            output,
            build_dependencies,
            host_dependencies: None,
            build_run_exports: Vec::new(),
            build_records: Vec::new(),
            host_records: Vec::new(),
        });
    }
    members.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(members)
}

fn validate_isolation_policy(
    name: &PackageName,
    deps: &DependencyMap<PackageName, PixiSpec>,
) -> Result<(), SolvePixiEnvironmentError> {
    let Some(specs) = deps.get(name) else {
        return Ok(());
    };
    let mut isolated = false;
    let mut shared = false;
    for spec in specs {
        if !matches!(spec.clone().into_source_or_binary(), Either::Left(_)) {
            continue;
        }
        if spec.build_dependency_mode() == Some(BuildDependencyMode::Isolated) {
            isolated = true;
        } else {
            shared = true;
        }
    }
    if isolated && shared {
        return Err(SolvePixiEnvironmentError::UnsupportedSharedSourceSolve(
            format!(
                "source dependency '{}' has conflicting build-dependency-mode policies in the solve group; use a consistent policy or separate solve groups",
                name.as_source()
            ),
        ));
    }
    Ok(())
}

fn is_isolated(name: &PackageName, deps: &DependencyMap<PackageName, PixiSpec>) -> bool {
    deps.iter()
        .find(|(n, _)| *n == name)
        .is_some_and(|(_, specs)| {
            specs
                .iter()
                .any(|spec| spec.build_dependency_mode() == Some(BuildDependencyMode::Isolated))
        })
}

fn member_id_for_record(record: &SourceRecord) -> MemberId {
    MemberId {
        name: record.package_record().name.clone(),
        manifest: record.manifest_source().to_string(),
        build: record.build_source().map(ToString::to_string),
        variant: record.variants.clone(),
    }
}

fn merge_member_requirements<'a>(
    deps: impl Iterator<Item = &'a Dependencies>,
) -> (
    DependencyMap<PackageName, PixiSpec>,
    DependencyMap<PackageName, BinarySpec>,
) {
    let mut dependencies = DependencyMap::default();
    let mut constraints = DependencyMap::default();
    for dep in deps {
        dependencies.extend(
            dep.dependencies
                .clone()
                .into_specs()
                .map(|(n, s)| (n, s.value)),
        );
        constraints.extend(
            dep.constraints
                .clone()
                .into_specs()
                .map(|(n, s)| (n, s.value)),
        );
    }
    (dependencies, constraints)
}

async fn solve_group(
    ctx: &mut ComputeCtx,
    env_spec: &crate::EnvironmentSpec,
    dependencies: DependencyMap<PackageName, PixiSpec>,
    constraints: DependencyMap<PackageName, BinarySpec>,
    source_repodata: Vec<Arc<SourceMetadata>>,
    installed: Vec<PixiRecord>,
    platform: Platform,
    virtual_packages: Vec<rattler_conda_types::GenericVirtualPackage>,
    strategy: SolveStrategy,
    exclude_newer: pixi_spec::ResolvedExcludeNewer,
) -> Result<Vec<PixiRecord>, SolvePixiEnvironmentError> {
    let mut source_specs = DependencyMap::default();
    let mut binary_specs = DependencyMap::default();
    for (name, spec) in dependencies.into_specs() {
        match spec.into_source_or_binary() {
            Either::Left(source) => {
                source_specs.insert(name, source);
            }
            Either::Right(binary) => {
                binary_specs.insert(name, binary);
            }
        }
    }
    ctx.compute(&SolveCondaKey::new(SolveCondaSpec {
        source_specs,
        binary_specs,
        constraints,
        dev_source_records: Vec::new(),
        source_repodata,
        installed,
        platform,
        channels: env_spec.channels.clone(),
        virtual_packages,
        strategy,
        channel_priority: env_spec.channel_priority,
        exclude_newer: Some(exclude_newer),
    }))
    .await
    .map(|records| (*records).clone())
    .map_err(map_solve_error)
}

fn map_solve_error(e: SolveCondaKeyError) -> SolvePixiEnvironmentError {
    match e {
        SolveCondaKeyError::Solve(a) => SolvePixiEnvironmentError::SolveError(a),
        SolveCondaKeyError::SpecConversion(a) => SolvePixiEnvironmentError::SpecConversionError(a),
        SolveCondaKeyError::Gateway(a) => SolvePixiEnvironmentError::QueryError(a),
        SolveCondaKeyError::CacheIndex(a) => SolvePixiEnvironmentError::CacheIndexError(a),
    }
}

fn collect_source_records(records: Vec<Arc<SourceRecord>>) -> Vec<Arc<SourceRecord>> {
    let mut queue = records;
    let mut seen = HashSet::new();
    let mut collected = Vec::new();
    while let Some(record) = queue.pop() {
        let identity = member_id_for_record(&record);
        if !seen.insert(identity) {
            continue;
        }
        for child in record
            .build_packages
            .iter()
            .chain(record.host_packages.iter())
        {
            if let Ok(PixiRecord::Source(source)) = child.clone().try_into_resolved() {
                queue.push(source);
            }
        }
        collected.push(record);
    }
    collected.sort_by(|a, b| member_id_for_record(a).cmp(&member_id_for_record(b)));
    collected
}

fn source_repodata_from_records(records: Vec<Arc<SourceRecord>>) -> Vec<Arc<SourceMetadata>> {
    let mut groups: HashMap<PinnedSourceCodeLocation, Vec<Arc<SourceRecord>>> = HashMap::new();
    for record in records {
        let loc = PinnedSourceCodeLocation::new(
            record.manifest_source().clone(),
            record.build_source().cloned(),
        );
        groups.entry(loc).or_default().push(record);
    }
    let mut grouped: Vec<_> = groups.into_iter().collect();
    for (_, records) in &mut grouped {
        records.sort_by(|a, b| {
            a.package_record()
                .name
                .as_normalized()
                .cmp(b.package_record().name.as_normalized())
                .then_with(|| a.variants.cmp(&b.variants))
        });
    }
    grouped.sort_by_key(|(source, _)| source.to_string());
    grouped
        .into_iter()
        .map(|(source, records)| Arc::new(SourceMetadata { source, records }))
        .collect()
}

fn project_dependency_closure(pool: &[PixiRecord], dependencies: &Dependencies) -> Vec<PixiRecord> {
    type QueueEntry = (PackageName, Option<String>);

    fn dependency_entries(dependency: &str) -> Vec<QueueEntry> {
        let base = PackageName::from_matchspec_str_unchecked(dependency);
        let mut entries = vec![(base, None)];
        if dependency.contains('[')
            && let Ok(spec) = MatchSpec::from_str(
                dependency,
                ParseMatchSpecOptions::lenient()
                    .with_repodata_revision(rattler_conda_types::RepodataRevision::V3),
            )
            && let PackageNameMatcher::Exact(name) = spec.name
        {
            entries.extend(
                spec.extras
                    .into_iter()
                    .flatten()
                    .map(|extra| (name.clone(), Some(extra))),
            );
        }
        entries
    }

    let by_name: HashMap<PackageName, PixiRecord> = pool
        .iter()
        .map(|record| (record.package_record().name.clone(), record.clone()))
        .collect();
    let mut queue = Vec::new();
    for (name, spec) in dependencies.dependencies.iter_specs() {
        queue.push((name.clone(), None));
        if let Some(extras) = spec.value.extras() {
            queue.extend(
                extras
                    .iter()
                    .cloned()
                    .map(|extra| (name.clone(), Some(extra))),
            );
        }
    }

    let mut queued: HashSet<QueueEntry> = queue.iter().cloned().collect();
    let mut seen_records = BTreeSet::new();
    let mut out = Vec::new();
    while let Some((name, extra)) = queue.pop() {
        let Some(record) = by_name.get(&name).cloned() else {
            continue;
        };
        for dep in &record.package_record().depends {
            for entry in dependency_entries(dep) {
                if queued.insert(entry.clone()) {
                    queue.push(entry);
                }
            }
        }
        if let Some(extra) = extra
            && let Some(extra_dependencies) = record.package_record().extra_depends.get(&extra)
        {
            for dep in extra_dependencies {
                for entry in dependency_entries(dep) {
                    if queued.insert(entry.clone()) {
                        queue.push(entry);
                    }
                }
            }
        }
        if seen_records.insert(name) {
            out.push(record);
        }
    }
    out.sort_by(|a, b| {
        a.package_record()
            .name
            .as_normalized()
            .cmp(b.package_record().name.as_normalized())
    });
    out
}

fn state_fingerprint(records: &[Arc<SourceRecord>]) -> Vec<String> {
    records
        .iter()
        .map(|r| {
            format!(
                "{}:{}:{}:{}:{:?}:{:?}",
                r.package_record().name.as_normalized(),
                r.package_record().version,
                r.package_record().build,
                r.identifier_hash,
                r.package_record().depends,
                r.package_record().constrains
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rattler_conda_types::{PackageRecord, VersionWithSource, package::DistArchiveIdentifier};
    use std::str::FromStr;
    use url::Url;

    fn rec(name: &str, deps: &[&str]) -> PixiRecord {
        PixiRecord::Binary(Arc::new(rattler_conda_types::RepoDataRecord {
            package_record: PackageRecord {
                name: PackageName::new_unchecked(name),
                version: VersionWithSource::from_str("1").unwrap(),
                build: "0".into(),
                build_number: 0,
                subdir: "linux-64".into(),
                depends: deps.iter().map(|s| s.to_string()).collect(),
                ..PackageRecord::new(
                    PackageName::new_unchecked(name),
                    VersionWithSource::from_str("1").unwrap(),
                    "0".into(),
                )
            },
            identifier: DistArchiveIdentifier::from_str(&format!("{name}-1-0.conda")).unwrap(),
            url: Url::parse(&format!("https://example.com/{name}-1-0.conda")).unwrap(),
            channel: None,
        }))
    }

    #[test]
    fn projects_only_member_dependency_closure() {
        let a = PackageName::new_unchecked("a");
        let c = PackageName::new_unchecked("c");
        let mut deps = Dependencies::default();
        deps.dependencies
            .insert(a, pixi_spec::PixiSpec::any().into());
        let projected = project_dependency_closure(
            &[rec("a", &["b >=1"]), rec("b", &[]), rec("c", &[])],
            &deps,
        );
        assert_eq!(
            projected
                .iter()
                .map(|r| r.package_record().name.as_normalized().to_string())
                .collect::<Vec<_>>(),
            ["a", "b"]
        );
        assert!(!projected.iter().any(|r| r.package_record().name == c));
    }

    #[test]
    fn every_member_projects_the_group_selected_version() {
        let mut first = Dependencies::default();
        first.dependencies.insert(
            PackageName::new_unchecked("first"),
            pixi_spec::PixiSpec::any().into(),
        );
        let mut second = Dependencies::default();
        second.dependencies.insert(
            PackageName::new_unchecked("second"),
            pixi_spec::PixiSpec::any().into(),
        );
        let pool = [
            rec("first", &["shared >=1"]),
            rec("second", &["shared >=1"]),
            rec("shared", &[]),
        ];

        for projected in [
            project_dependency_closure(&pool, &first),
            project_dependency_closure(&pool, &second),
        ] {
            let selected = projected
                .iter()
                .find(|record| record.package_record().name.as_normalized() == "shared")
                .expect("shared dependency must be projected");
            assert_eq!(selected.package_record().version.to_string(), "1");
        }
    }

    #[test]
    fn conflicting_isolation_policies_are_rejected() {
        let name = PackageName::new_unchecked("demo");
        let mut deps = DependencyMap::default();
        deps.insert(
            name.clone(),
            PixiSpec::PathSource(Box::new(pixi_spec::PathSourceSpec {
                path: "shared".into(),
                matchspec: Default::default(),
            })),
        );
        deps.insert(
            name,
            PixiSpec::PathSource(Box::new(pixi_spec::PathSourceSpec {
                path: "isolated".into(),
                matchspec: pixi_spec::MatchspecFields {
                    build_dependency_mode: Some(BuildDependencyMode::Isolated),
                    ..Default::default()
                },
            })),
        );
        assert!(validate_isolation_policy(&PackageName::new_unchecked("demo"), &deps).is_err());
    }

    #[test]
    fn isolated_source_dependency_does_not_join_group() {
        let name = PackageName::new_unchecked("demo");
        let mut deps = DependencyMap::default();
        deps.insert(
            name.clone(),
            PixiSpec::PathSource(Box::new(pixi_spec::PathSourceSpec {
                path: "source".into(),
                matchspec: pixi_spec::MatchspecFields {
                    build_dependency_mode: Some(BuildDependencyMode::Isolated),
                    ..Default::default()
                },
            })),
        );
        assert!(is_isolated(&name, &deps));
    }
}

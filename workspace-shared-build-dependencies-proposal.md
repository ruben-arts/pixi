# Proposal: shared workspace dependency policy for source builds

## Problem

When Pixi builds source packages inside a workspace, the package build and host environments can currently resolve dependency versions independently from the environment that consumes the package.

For example, a workspace environment may say:

```toml
[dependencies]
numpy = "2.1.*"
my-extension = { path = "./my-extension" }
```

while `my-extension` has, or its backend infers:

```toml
[package.host-dependencies]
numpy = "*"
```

In many workspace builds, especially ROS-style source workspaces, this should not mean “pick any numpy for the package host environment”. It should mean “build this package against the same numpy selected for this workspace environment”.

The same issue applies to dependencies between source packages. A backend such as `pixi-build-ros` can infer dependencies from `package.xml`, but it should not also have to decide whether each dependency should come from another workspace source package, a git source package, a path source package, or a channel package. Pixi has the workspace/environment context and should provide that policy.

## Goal

Add a way to request that source packages participating in an environment are built in the dependency context of that environment.

In that mode, if an environment selects `numpy = 2.1.*`, source packages built for that environment should use that same compatible `numpy` in their build/host environments where they depend on `numpy`, unless that environment's dependency entry explicitly opts out.

This is not just “copy `[dependencies]` into every build environment”. The goal is a globally consistent workspace/environment solve that includes source-package build, host, and run requirements, including metadata effects such as run-exports.

## Possible manifest syntax

Workspace-wide default:

```toml
[workspace.build]
dependency-mode = "shared"
```

Environment-specific override:

```toml
[environments.default]
features = ["default"]
build-dependency-mode = "shared"
```

Per-dependency escape hatch in an environment:

```toml
[dependencies]
legacy-extension = { path = "./legacy-extension", build-dependency-mode = "isolated" }
```

or, if the package comes from a feature used only by one environment:

```toml
[feature.legacy.dependencies]
legacy-extension = { path = "./legacy-extension", build-dependency-mode = "isolated" }
```

Possible modes:

```toml
build-dependency-mode = "shared"   # build this source dependency in the environment's dependency context
build-dependency-mode = "isolated" # build this source dependency with package-local build/host solves
```

The exact names are bikesheddable. The important part is that the default policy can be selected at environment/workspace level, while the opt-out is attached to the environment dependency edge that pulls the source package in.

## Desired semantics

When `shared` mode is enabled for an environment:

- Pixi treats the environment dependencies and participating source-package dependencies as one consistency problem.
- If an environment constrains a package, that constraint also applies to matching build/host dependencies of source packages built for that environment.
- If the environment contains source/path/git packages for dependency names, those source packages are preferred for source-package build/host/run dependencies where applicable.
- Build backends can keep reporting dependencies by name. They should not need to inject path/git/channel decisions themselves.
- The final lock file should describe a globally compatible set of:
  - environment dependencies,
  - source package build dependencies,
  - source package host dependencies,
  - source package run dependencies,
  - run-exports and run constraints derived from the selected build/host packages.

Example:

```toml
[dependencies]
numpy = "2.1.*"
my-extension = { path = "./my-extension" }
```

If `my-extension` has:

```toml
[package.host-dependencies]
numpy = "*"
```

then the host environment for `my-extension` should resolve with the workspace/environment choice of `numpy = 2.1.*`.

## Run-exports and `pin-compatible`

Run-exports make this more than a simple upfront merge of dependency specs.

Some run-exports are static, but others use `pin-compatible`, which can only be resolved after the relevant build or host environment has been solved. For example:

```toml
[package.run-exports.weak]
numpy = { pin-compatible = { lower-bound = "x.x", upper-bound = "x" } }
```

The backend can report this symbolically, but Pixi needs the concrete `numpy` version/build from the solved compatibility environment before it can turn it into a concrete runtime constraint.

Current Pixi behavior is staged roughly like this for one source package:

1. ask the backend for `CondaOutput` metadata with symbolic dependencies/run-exports;
2. solve the build environment;
3. extract build-package run-exports;
4. solve the host environment, with access to build records for `pin-compatible`;
5. extract host-package run-exports;
6. resolve run dependencies and the package's own run-exports, with access to build + host records.

A shared workspace mode does not need to require one literal solver invocation. It can still use staged or fixpoint solving internally. The user-visible contract should be:

> all source packages selected for an environment are resolved as one global consistency problem, and derived metadata such as run-exports and `pin-compatible` constraints must be validated against that same environment.

So internally Pixi may need to:

1. collect backend metadata for selected source packages;
2. perform a shared solve for environment/build/host requirements where possible;
3. materialize `pin-compatible` constraints after the relevant build/host records are known;
4. re-check or re-solve until the run-exports are compatible with the environment;
5. fail with a useful conflict if no globally consistent solution exists.

## Validation behavior

If a package's generated run-exports are incompatible with the shared environment, Pixi should fail during solving/locking with an actionable error.

For example, if the environment requires:

```toml
[dependencies]
numpy = "2.1.*"
```

but a source package's resolved build/host metadata exports a runtime constraint requiring `numpy <2`, the solve should fail before or during lock-file generation, rather than discovering the incompatibility only while building a later package.

The error should point to:

- the source package that generated the incompatible run-export,
- the dependency that caused the exported constraint,
- the environment constraint it conflicts with,
- and the escape hatch, e.g. marking that environment dependency as isolated if appropriate.

## Isolation escape hatch

Some packages cannot safely be built against the shared workspace dependency set in every environment. They may need an older compiler, an older ABI, or a dependency version that intentionally differs from one environment but not another.

The escape hatch should therefore live on the dependency entry in the environment, not in the package table. The package itself should not have to declare that it is always isolated; the consuming environment should decide how it wants that package built.

For example:

```toml
[dependencies]
legacy-extension = { path = "./legacy-extension", build-dependency-mode = "isolated" }
```

And another environment could still build the same package in shared mode:

```toml
[feature.modern.dependencies]
legacy-extension = { path = "./legacy-extension", build-dependency-mode = "shared" }
```

In isolated mode for a dependency edge:

- that source package's build/host environments are solved from the package's own declared or backend-inferred requirements;
- the consuming environment's dependency choices are not injected into that package's build/host solve;
- the produced package metadata, including run-exports, is still validated against the environment that consumes it.

This keeps isolation available where a specific environment needs it, while allowing the same source package to participate in shared mode in other environments.

## Open questions

- What should the exact per-dependency syntax be for opting into or out of shared build dependency mode?
- Should shared dependency choices be hard constraints or solver preferences?
- Should the policy apply only to source/path/git packages, or also to packages rebuilt from recipes?
- How should Pixi present conflicts caused by materialized `pin-compatible` run-exports?
- Does shared mode require a fixpoint solve, or can it be implemented as staged solves plus final validation?

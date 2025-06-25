# pixi.toml Schema Documentation

## Activation
A description of steps performed when an environment is activated

### `activation.scripts`
**Type:** List[String]

The scripts to run when the environment is activated
```toml
[activation]
scripts = ["activate.sh"]
```
```toml
[activation]
scripts = ["activate.bat"]
```

### `activation.env`
**Type:** Object

A map of environment variables to values, used in the activation of the environment. These will be set in the shell. Thus these variables are shell specific. Using '$' might not expand to a value in different shells.
```toml
[activation.env]
key = "value"
ARGUMENT = "value"
```

## Build
### `build.backend.version`
**Type:** String

The version of the package in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format

### `build.backend.build`
**Type:** String

The build string of the package

### `build.backend.build-number`
**Type:** String

The build number of the package, can be a spec like `>=1` or `<=10` or `1`

### `build.backend.file-name`
**Type:** String

The file name of the package

### `build.backend.channel`
**Type:** String

The channel the packages needs to be fetched from
```toml
[build.backend]
channel = "conda-forge"
```
```toml
[build.backend]
channel = "pytorch"
```
```toml
[build.backend]
channel = "https://repo.prefix.dev/conda-forge"
```

### `build.backend.subdir`
**Type:** String

The subdir of the package, also known as platform

### `build.backend.license`
**Type:** String

The license of the package

### `build.backend.path`
**Type:** String

The path to the package

### `build.backend.url`
**Type:** String

The URL to the package

### `build.backend.md5`
**Type:** String

The md5 hash of the package

### `build.backend.sha256`
**Type:** String

The sha256 hash of the package

### `build.backend.git`
**Type:** String

The git URL to the repo

### `build.backend.rev`
**Type:** String

A git SHA revision to use

### `build.backend.tag`
**Type:** String

A git tag to use

### `build.backend.branch`
**Type:** String

A git branch to use

### `build.backend.subdirectory`
**Type:** String

A subdirectory to use in the repo

### `build.backend.name`
**Type:** String

The name of the build backend package

### `build.channels`
**Type:** List[String | String | [ChannelInlineTable](#channelinlinetable)]

The `conda` channels that are used to fetch the build backend from

### `build.additional-dependencies`
**Type:** Object

Additional dependencies to install alongside the build backend

### `build.configuration`
**Type:** Object

The configuration of the build backend

## BuildBackend
### `build-backend.version`
**Type:** String

The version of the package in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format

### `build-backend.build`
**Type:** String

The build string of the package

### `build-backend.build-number`
**Type:** String

The build number of the package, can be a spec like `>=1` or `<=10` or `1`

### `build-backend.file-name`
**Type:** String

The file name of the package

### `build-backend.channel`
**Type:** String

The channel the packages needs to be fetched from
```toml
[build-backend]
channel = "conda-forge"
```
```toml
[build-backend]
channel = "pytorch"
```
```toml
[build-backend]
channel = "https://repo.prefix.dev/conda-forge"
```

### `build-backend.subdir`
**Type:** String

The subdir of the package, also known as platform

### `build-backend.license`
**Type:** String

The license of the package

### `build-backend.path`
**Type:** String

The path to the package

### `build-backend.url`
**Type:** String

The URL to the package

### `build-backend.md5`
**Type:** String

The md5 hash of the package

### `build-backend.sha256`
**Type:** String

The sha256 hash of the package

### `build-backend.git`
**Type:** String

The git URL to the repo

### `build-backend.rev`
**Type:** String

A git SHA revision to use

### `build-backend.tag`
**Type:** String

A git tag to use

### `build-backend.branch`
**Type:** String

A git branch to use

### `build-backend.subdirectory`
**Type:** String

A subdirectory to use in the repo

### `build-backend.name`
**Type:** String

The name of the build backend package

## ChannelInlineTable
A precise description of a `conda` channel, with an optional priority.

### `channel-inline-table.channel`
**Type:** String | String

**Required:** Yes

The channel the packages needs to be fetched from

### `channel-inline-table.priority`
**Type:** Integer

The priority of the channel

## ChannelPriority
The priority of the channel.

## DependsOn
The dependencies of a task.

### `depends-on.task`
**Type:** String

**Required:** Yes

A valid task name.

### `depends-on.args`
**Type:** List[String]

The arguments to pass to the task

### `depends-on.environment`
**Type:** String

The environment to use for the task

## Environment
A composition of the dependencies of features which can be activated to run tasks or provide a shell

### `environment.features`
**Type:** List[String]

The features that define the environment

### `environment.solve-group`
**Type:** String

The group name for environments that should be solved together

### `environment.no-default-feature`
**Type:** Boolean

**Default:** `False`

Whether to add the default feature to this environment

## Feature
A composable aspect of the project which can contribute dependencies and tasks to an environment

### `feature.channels`
**Type:** List[String | String | [ChannelInlineTable](#channelinlinetable)]

The `conda` channels that can be considered when solving environments containing this feature

### `feature.channel-priority`
**Type:** [ChannelPriority](#channelpriority)

The type of channel priority that is used in the solve.- 'strict': only take the package from the channel it exist in first.- 'disabled': group all dependencies together as if there is no channel difference.
```toml
[feature]
channel-priority = "strict"
```
```toml
[feature]
channel-priority = "disabled"
```

### `feature.platforms`
**Type:** List[[Platform](#platform)]

The platforms that the feature supports: a union of all features combined in one environment is used for the environment.

### `feature.dependencies`
**Type:** Object

The `conda` dependencies, consisting of a package name and a requirement in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format

### `feature.host-dependencies`
**Type:** Object

The host `conda` dependencies, used in the build process. See https://pixi.sh/latest/build/dependency_types/ for more information.
```toml
[feature.host-dependencies]
python = ">=3.8"
```

### `feature.build-dependencies`
**Type:** Object

The build `conda` dependencies, used in the build process. See https://pixi.sh/latest/build/dependency_types/ for more information.

### `feature.pypi-dependencies`
**Type:** Object

The PyPI dependencies of this feature

### `feature.tasks`
**Type:** Object

The tasks provided by this feature

### `feature.activation`
**Type:** [Activation](#activation)

The scripts used on the activation of environments using this feature

### `feature.system-requirements`
**Type:** [SystemRequirements](#systemrequirements)

The system requirements of this feature

### `feature.target`
**Type:** Object

Machine-specific aspects of this feature
```toml
[feature.target]
linux = "{'dependencies': {'python': '3.8'}}"
```

### `feature.pypi-options`
**Type:** [PyPIOptions](#pypioptions)

Options related to PyPI indexes for this feature

## FindLinksPath
The path to the directory containing packages

### `find-links-path.path`
**Type:** String

Path to the directory of packages
```toml
[find-links-path]
path = "./links"
```

## FindLinksURL
The URL to the html file containing href-links to packages

### `find-links-u-r-l.url`
**Type:** String

URL to html file with href-links to packages
```toml
[find-links-u-r-l]
url = "https://simple-index-is-here.com"
```

## LibcFamily
### `libc-family.family`
**Type:** String

The family of the `libc`
```toml
[libc-family]
family = "glibc"
```
```toml
[libc-family]
family = "musl"
```

### `libc-family.version`
**Type:** Number | String

The version of `libc`

## MatchspecTable
A precise description of a `conda` package version.

### `matchspec-table.version`
**Type:** String

The version of the package in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format

### `matchspec-table.build`
**Type:** String

The build string of the package

### `matchspec-table.build-number`
**Type:** String

The build number of the package, can be a spec like `>=1` or `<=10` or `1`

### `matchspec-table.file-name`
**Type:** String

The file name of the package

### `matchspec-table.channel`
**Type:** String

The channel the packages needs to be fetched from
```toml
[matchspec-table]
channel = "conda-forge"
```
```toml
[matchspec-table]
channel = "pytorch"
```
```toml
[matchspec-table]
channel = "https://repo.prefix.dev/conda-forge"
```

### `matchspec-table.subdir`
**Type:** String

The subdir of the package, also known as platform

### `matchspec-table.license`
**Type:** String

The license of the package

### `matchspec-table.path`
**Type:** String

The path to the package

### `matchspec-table.url`
**Type:** String

The URL to the package

### `matchspec-table.md5`
**Type:** String

The md5 hash of the package

### `matchspec-table.sha256`
**Type:** String

The sha256 hash of the package

### `matchspec-table.git`
**Type:** String

The git URL to the repo

### `matchspec-table.rev`
**Type:** String

A git SHA revision to use

### `matchspec-table.tag`
**Type:** String

A git tag to use

### `matchspec-table.branch`
**Type:** String

A git branch to use

### `matchspec-table.subdirectory`
**Type:** String

A subdirectory to use in the repo

## Package
The package's metadata information.

### `package.name`
**Type:** String

The name of the package

### `package.version`
**Type:** String

The version of the project; we advise use of [SemVer](https://semver.org)
```toml
[package]
version = "1.2.3"
```

### `package.description`
**Type:** String

A short description of the project

### `package.authors`
**Type:** List[String]

The authors of the project
```toml
[package]
authors = ["John Doe <j.doe@prefix.dev>"]
```

### `package.license`
**Type:** String

The license of the project; we advise using an [SPDX](https://spdx.org/licenses/) identifier.

### `package.license-file`
**Type:** String

The path to the license file of the project

### `package.readme`
**Type:** String

The path to the readme file of the project

### `package.homepage`
**Type:** String

The URL of the homepage of the project

### `package.repository`
**Type:** String

The URL of the repository of the project

### `package.documentation`
**Type:** String

The URL of the documentation of the project

### `package.build.backend.version`
**Type:** String

The version of the package in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format

### `package.build.backend.build`
**Type:** String

The build string of the package

### `package.build.backend.build-number`
**Type:** String

The build number of the package, can be a spec like `>=1` or `<=10` or `1`

### `package.build.backend.file-name`
**Type:** String

The file name of the package

### `package.build.backend.channel`
**Type:** String

The channel the packages needs to be fetched from
```toml
[package.build.backend]
channel = "conda-forge"
```
```toml
[package.build.backend]
channel = "pytorch"
```
```toml
[package.build.backend]
channel = "https://repo.prefix.dev/conda-forge"
```

### `package.build.backend.subdir`
**Type:** String

The subdir of the package, also known as platform

### `package.build.backend.license`
**Type:** String

The license of the package

### `package.build.backend.path`
**Type:** String

The path to the package

### `package.build.backend.url`
**Type:** String

The URL to the package

### `package.build.backend.md5`
**Type:** String

The md5 hash of the package

### `package.build.backend.sha256`
**Type:** String

The sha256 hash of the package

### `package.build.backend.git`
**Type:** String

The git URL to the repo

### `package.build.backend.rev`
**Type:** String

A git SHA revision to use

### `package.build.backend.tag`
**Type:** String

A git tag to use

### `package.build.backend.branch`
**Type:** String

A git branch to use

### `package.build.backend.subdirectory`
**Type:** String

A subdirectory to use in the repo

### `package.build.backend.name`
**Type:** String

The name of the build backend package

### `package.build.channels`
**Type:** List[String | String | [ChannelInlineTable](#channelinlinetable)]

The `conda` channels that are used to fetch the build backend from

### `package.build.additional-dependencies`
**Type:** Object

Additional dependencies to install alongside the build backend

### `package.build.configuration`
**Type:** Object

The configuration of the build backend

### `package.host-dependencies`
**Type:** Object

The host `conda` dependencies, used in the build process. See https://pixi.sh/latest/build/dependency_types/ for more information.
```toml
[package.host-dependencies]
python = ">=3.8"
```

### `package.build-dependencies`
**Type:** Object

The build `conda` dependencies, used in the build process. See https://pixi.sh/latest/build/dependency_types/ for more information.

### `package.run-dependencies`
**Type:** Object

The `conda` dependencies required at runtime. See https://pixi.sh/latest/build/dependency_types/ for more information.

### `package.target`
**Type:** Object

Machine-specific aspects of the package
```toml
[package.target]
linux = "{'host-dependencies': {'python': '3.8'}}"
```

## Platform
A supported operating system and processor architecture pair.

## PyPIGitBranchRequirement
### `py-p-i-git-branch-requirement.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-git-branch-requirement.git`
**Type:** String

The `git` URL to the repo e.g https://github.com/prefix-dev/pixi

### `py-p-i-git-branch-requirement.subdirectory`
**Type:** String

The subdirectory in the repo, a path from the root of the repo.

### `py-p-i-git-branch-requirement.branch`
**Type:** String

A `git` branch to use

## PyPIGitRevRequirement
### `py-p-i-git-rev-requirement.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-git-rev-requirement.git`
**Type:** String

The `git` URL to the repo e.g https://github.com/prefix-dev/pixi

### `py-p-i-git-rev-requirement.subdirectory`
**Type:** String

The subdirectory in the repo, a path from the root of the repo.

### `py-p-i-git-rev-requirement.rev`
**Type:** String

A `git` SHA revision to use

## PyPIGitTagRequirement
### `py-p-i-git-tag-requirement.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-git-tag-requirement.git`
**Type:** String

The `git` URL to the repo e.g https://github.com/prefix-dev/pixi

### `py-p-i-git-tag-requirement.subdirectory`
**Type:** String

The subdirectory in the repo, a path from the root of the repo.

### `py-p-i-git-tag-requirement.tag`
**Type:** String

A `git` tag to use

## PyPIOptions
Options that determine the behavior of PyPI package resolution and installation

### `py-p-i-options.index-url`
**Type:** String

PyPI registry that should be used as the primary index
```toml
[py-p-i-options]
index-url = "https://pypi.org/simple"
```

### `py-p-i-options.extra-index-urls`
**Type:** List[String]

Additional PyPI registries that should be used as extra indexes
```toml
[py-p-i-options]
extra-index-urls = ["https://pypi.org/simple"]
```

### `py-p-i-options.find-links`
**Type:** List[[FindLinksPath](#findlinkspath) | [FindLinksURL](#findlinksurl)]

Paths to directory containing
```toml
[py-p-i-options]
find-links = ["https://pypi.org/simple"]
```

### `py-p-i-options.no-build-isolation`
**Type:** Boolean | List[String]

Packages that should NOT be isolated during the build process
```toml
[py-p-i-options]
no-build-isolation = ["numpy"]
```
```toml
[py-p-i-options]
no-build-isolation = [true]
```

### `py-p-i-options.index-strategy`
**Type:** String | String | String

The strategy to use when resolving packages from multiple indexes
```toml
[py-p-i-options]
index-strategy = "first-index"
```
```toml
[py-p-i-options]
index-strategy = "unsafe-first-match"
```
```toml
[py-p-i-options]
index-strategy = "unsafe-best-match"
```

### `py-p-i-options.no-build`
**Type:** Boolean | List[String]

Packages that should NOT be built
```toml
[py-p-i-options]
no-build = ["true"]
```
```toml
[py-p-i-options]
no-build = ["false"]
```

## PyPIPathRequirement
### `py-p-i-path-requirement.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-path-requirement.path`
**Type:** String

A path to a local source or wheel

### `py-p-i-path-requirement.editable`
**Type:** Boolean

If `true` the package will be installed as editable

### `py-p-i-path-requirement.subdirectory`
**Type:** String

The subdirectory in the repo, a path from the root of the repo.

## PyPIUrlRequirement
### `py-p-i-url-requirement.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-url-requirement.url`
**Type:** String

A URL to a remote source or wheel

## PyPIVersion
### `py-p-i-version.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-version.version`
**Type:** String

The version of the package in [PEP 440](https://www.python.org/dev/peps/pep-0440/) format

### `py-p-i-version.index`
**Type:** String

The index to fetch the package from

## S3Options
Options related to S3 for this project

### `s3-options.endpoint-url`
**Type:** String

**Required:** Yes

The endpoint URL to use for the S3 client
```toml
[s3-options]
endpoint-url = "https://s3.eu-central-1.amazonaws.com"
```

### `s3-options.region`
**Type:** String

**Required:** Yes

The region to use for the S3 client
```toml
[s3-options]
region = "eu-central-1"
```

### `s3-options.force-path-style`
**Type:** Boolean

**Required:** Yes

Whether to force path style for the S3 client

## SystemRequirements
Platform-specific requirements

### `system-requirements.linux`
**Type:** Number | String

The minimum version of the Linux kernel

### `system-requirements.unix`
**Type:** Boolean | String

Whether the project supports UNIX
```toml
[system-requirements]
unix = "true"
```

### `system-requirements.libc`
**Type:** [LibcFamily](#libcfamily) | Number | String

The minimum version of `libc`

### `system-requirements.cuda`
**Type:** Number | String

The minimum version of CUDA

### `system-requirements.archspec`
**Type:** String

The architecture the project supports

### `system-requirements.macos`
**Type:** Number | String

The minimum version of MacOS

## Target
A machine-specific configuration of dependencies and tasks

### `target.dependencies`
**Type:** Object

The `conda` dependencies, consisting of a package name and a requirement in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format

### `target.host-dependencies`
**Type:** Object

The host `conda` dependencies, used in the build process. See https://pixi.sh/latest/build/dependency_types/ for more information.
```toml
[target.host-dependencies]
python = ">=3.8"
```

### `target.build-dependencies`
**Type:** Object

The build `conda` dependencies, used in the build process. See https://pixi.sh/latest/build/dependency_types/ for more information.

### `target.pypi-dependencies`
**Type:** Object

The PyPI dependencies for this target

### `target.tasks`
**Type:** Object

The tasks of the target

### `target.activation`
**Type:** [Activation](#activation)

The scripts used on the activation of the project for this target

## TaskArgs
The arguments of a task.

### `task-args.arg`
**Type:** String

**Required:** Yes


### `task-args.default`
**Type:** String

The default value of the argument

## TaskInlineTable
A precise definition of a task.

### `task-inline-table.cmd`
**Type:** List[String] | String

A shell command to run the task in the limited, but cross-platform `bash`-like `deno_task_shell`. See the documentation for [supported syntax](https://pixi.sh/latest/environments/advanced_tasks/#syntax)

### `task-inline-table.cwd`
**Type:** String

The working directory to run the task

### `task-inline-table.depends-on`
**Type:** List[String] | String

The tasks that this task depends on. Environment variables will **not** be expanded. Deprecated in favor of `depends-on` from v0.21.0 onward.

### `task-inline-table.depends-on`
**Type:** List[[DependsOn](#dependson) | String] | [DependsOn](#dependson) | String

The tasks that this task depends on. Environment variables will **not** be expanded.

### `task-inline-table.inputs`
**Type:** List[String]

A list of `.gitignore`-style glob patterns that should be watched for changes before this command is run. Environment variables _will_ be expanded.

### `task-inline-table.outputs`
**Type:** List[String]

A list of `.gitignore`-style glob patterns that are generated by this command. Environment variables _will_ be expanded.

### `task-inline-table.env`
**Type:** Object

A map of environment variables to values, used in the task, these will be overwritten by the shell.
```toml
[task-inline-table.env]
key = "value"
ARGUMENT = "value"
```

### `task-inline-table.description`
**Type:** String

A short description of the task
```toml
[task-inline-table]
description = "Build the project"
```

### `task-inline-table.clean-env`
**Type:** Boolean

Whether to run in a clean environment, removing all environment variables except those defined in `env` and by pixi itself.

### `task-inline-table.args`
**Type:** List[[TaskArgs](#taskargs) | String]

The arguments to pass to the task
```toml
[task-inline-table]
args = ["arg1"]
```
```toml
[task-inline-table]
args = ["arg2"]
```

## Workspace
The project's metadata information.

### `workspace.name`
**Type:** String

The name of the project; we advise use of the name of the repository

### `workspace.version`
**Type:** String

The version of the project; we advise use of [SemVer](https://semver.org)
```toml
[workspace]
version = "1.2.3"
```

### `workspace.description`
**Type:** String

A short description of the project

### `workspace.authors`
**Type:** List[String]

The authors of the project
```toml
[workspace]
authors = ["John Doe <j.doe@prefix.dev>"]
```

### `workspace.channels`
**Type:** List[String | String | [ChannelInlineTable](#channelinlinetable)]

**Required:** Yes

The `conda` channels that can be used in the project. Unless overridden by `priority`, the first channel listed will be preferred.

### `workspace.channel-priority`
**Type:** [ChannelPriority](#channelpriority)

The type of channel priority that is used in the solve.- 'strict': only take the package from the channel it exist in first.- 'disabled': group all dependencies together as if there is no channel difference.
```toml
[workspace]
channel-priority = "strict"
```
```toml
[workspace]
channel-priority = "disabled"
```

### `workspace.exclude-newer`
**Type:** String

Exclude any package newer than this date
```toml
[workspace]
exclude-newer = "2023-11-03"
```
```toml
[workspace]
exclude-newer = "2023-11-03T03:33:12Z"
```

### `workspace.platforms`
**Type:** List[[Platform](#platform)]

The platforms that the project supports

### `workspace.license`
**Type:** String

The license of the project; we advise using an [SPDX](https://spdx.org/licenses/) identifier.

### `workspace.license-file`
**Type:** String

The path to the license file of the project

### `workspace.readme`
**Type:** String

The path to the readme file of the project

### `workspace.homepage`
**Type:** String

The URL of the homepage of the project

### `workspace.repository`
**Type:** String

The URL of the repository of the project

### `workspace.documentation`
**Type:** String

The URL of the documentation of the project

### `workspace.conda-pypi-map`
**Type:** Object

The `conda` to PyPI mapping configuration

### `workspace.pypi-options`
**Type:** [PyPIOptions](#pypioptions)

Options related to PyPI indexes for this project

### `workspace.s3-options`
**Type:** Object

Options related to S3 for this project

### `workspace.preview`
**Type:** List[String | String] | Boolean

Defines the enabling of preview features of the project

### `workspace.build-variants`
**Type:** Object

The build variants of the project

### `workspace.requires-pixi`
**Type:** String

The required version spec for pixi itself to resolve and build the project.
```toml
[workspace]
requires-pixi = ">=0.40"
```

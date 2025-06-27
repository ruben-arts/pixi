# pixi.toml Schema Documentation

## workspace
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
**Type:** List[String | Url | [Channel](#channel)]

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
**Type:** Url

The URL of the homepage of the project

### `workspace.repository`
**Type:** Url

The URL of the repository of the project

### `workspace.documentation`
**Type:** Url

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

## package
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
**Type:** Url

The URL of the homepage of the project

### `package.repository`
**Type:** Url

The URL of the repository of the project

### `package.documentation`
**Type:** Url

The URL of the documentation of the project

### `package.build`
**Type:** [Build](#build)

### `package.host-dependencies`
**Type:** Object

Deprecated in favor of `package.host-dependencies`
```toml
[package.host-dependencies]
python = ">=3.8"
```

### `package.build-dependencies`
**Type:** Object

Deprecated in favor of `package.build-dependencies`

### `package.run-dependencies`
**Type:** Object

Deprecated in favor of `package.run-dependencies`

### `package.target`
**Type:** Object

Machine-specific aspects of the package
```toml
[package.target]
linux = "{'host-dependencies': {'python': '3.8'}}"
```

## py-p-i-options
Options that determine the behavior of PyPI package resolution and installation

### `pypi-options.index-url`
**Type:** String

PyPI registry that should be used as the primary index
```toml
[pypi-options]
index-url = "https://pypi.org/simple"
```

### `pypi-options.extra-index-urls`
**Type:** List[String]

Additional PyPI registries that should be used as extra indexes
```toml
[pypi-options]
extra-index-urls = ["https://pypi.org/simple"]
```

### `pypi-options.find-links`
**Type:** List[[FindLinksPath](#findlinkspath) | [FindLinksURL](#findlinksurl)]

Paths to directory containing
```toml
[pypi-options]
find-links = ["https://pypi.org/simple"]
```

### `pypi-options.no-build-isolation`
**Type:** Boolean | List[String]

Packages that should NOT be isolated during the build process
```toml
[pypi-options]
no-build-isolation = ["numpy"]
```
```toml
[pypi-options]
no-build-isolation = [true]
```

### `pypi-options.index-strategy`
**Type:** String | String | String

The strategy to use when resolving packages from multiple indexes
```toml
[pypi-options]
index-strategy = "first-index"
```
```toml
[pypi-options]
index-strategy = "unsafe-first-match"
```
```toml
[pypi-options]
index-strategy = "unsafe-best-match"
```

### `pypi-options.no-build`
**Type:** Boolean | List[String]

Packages that should NOT be built
```toml
[pypi-options]
no-build = ["true"]
```
```toml
[pypi-options]
no-build = ["false"]
```

## system-requirements
Platform-specific requirements

### `system-requirements.linux`
**Type:** Number | String

The minimum version of the Linux kernel
```toml
[system-requirements]
linux = "5.4"
```
```toml
[system-requirements]
linux = "4.19"
```

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
```toml
[system-requirements]
libc = {"family": "glibc", "version": "2.28"}
```
```toml
[system-requirements]
libc = 2.17
```

### `system-requirements.cuda`
**Type:** Number | String

The minimum version of CUDA
```toml
[system-requirements]
cuda = "12.0"
```

### `system-requirements.archspec`
**Type:** String

The architecture the project supports

### `system-requirements.macos`
**Type:** Number | String

The minimum version of MacOS
```toml
[system-requirements]
macos = "11.0"
```

## activation
A description of steps performed when an environment is activated

### `activation.scripts`
**Type:** List[String]

The scripts to run when the environment is activated
```toml
[activation]
scripts = ["activate.sh", "setup.sh"]
```
```toml
[activation]
scripts = ["activate.bat", "setup.bat"]
```

### `activation.env`
**Type:** Object

A map of environment variables to values, used in the activation of the environment. These will be set in the shell. Thus these variables are shell specific. Using '$' might not expand to a value in different shells.
```toml
[activation.env]
key = "value"
ARGUMENT = "value"
```

## activation
A description of steps performed when an environment is activated

### `activation.scripts`
**Type:** List[String]

The scripts to run when the environment is activated
```toml
[activation]
scripts = ["activate.sh", "setup.sh"]
```
```toml
[activation]
scripts = ["activate.bat", "setup.bat"]
```

### `activation.env`
**Type:** Object

A map of environment variables to values, used in the activation of the environment. These will be set in the shell. Thus these variables are shell specific. Using '$' might not expand to a value in different shells.
```toml
[activation.env]
key = "value"
ARGUMENT = "value"
```

## build
### `build.backend`
**Type:** [BuildBackend](#build-backend)

### `build.channels`
**Type:** List[String | Url | [Channel](#channel)]

The `conda` channels that are used to fetch the build backend from

### `build.additional-dependencies`
**Type:** Object

Additional dependencies to install alongside the build backend

### `build.configuration`
**Type:** Object

The configuration of the build backend

## build-backend
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

## channel
A precise description of a `conda` channel, with an optional priority.

### `channel.channel`
**Type:** String | Url

**Required:** Yes

The channel the packages needs to be fetched from

### `channel.priority`
**Type:** Integer

The priority of the channel

## channel-priority
The priority of the channel.

## depends-on
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

## environment
A composition of the dependencies of features which can be activated to run tasks or provide a shell

### `environment.features`
**Type:** List[String]

The features that define the environment
```toml
[environment]
features = ["feature1", "feature2"]
```

### `environment.solve-group`
**Type:** String

The group name for environments that should be solved together
```toml
[environment]
solve-group = "default"
```

### `environment.no-default-feature`
**Type:** Boolean

**Default:** `False`

Whether to add the default feature to this environment
```toml
[environment]
no-default-feature = "true"
```

## feature
A composable aspect of the project which can contribute dependencies and tasks to an environment

### `feature.channels`
**Type:** List[String | Url | [Channel](#channel)]

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
```toml
[feature.dependencies]
python = ">=3.8"
numpy = "==1.21.0"
requests = ">=2.25, <3"
channel = "pytorch"
pytorch = "1.9.0"
cmake = "{'build': 'h5b0a8f6_0', 'version': '>=3.18'}"
```

### `feature.host-dependencies`
**Type:** Object

Deprecated in favor of `package.host-dependencies`
```toml
[feature.host-dependencies]
python = ">=3.8"
```

### `feature.build-dependencies`
**Type:** Object

Deprecated in favor of `package.build-dependencies`

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

## find-links-path
The path to the directory containing packages

### `find-links-path.path`
**Type:** String

Path to the directory of packages
```toml
[find-links-path]
path = "./links"
```

## find-links-u-r-l
The URL to the html file containing href-links to packages

### `find-links-u-r-l.url`
**Type:** String

URL to html file with href-links to packages
```toml
[find-links-u-r-l]
url = "https://simple-index-is-here.com"
```

## libc-family
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
```toml
[libc-family]
version = "2.28"
```
```toml
[libc-family]
version = "2.17"
```

## matchspec-table
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

## package
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
**Type:** Url

The URL of the homepage of the project

### `package.repository`
**Type:** Url

The URL of the repository of the project

### `package.documentation`
**Type:** Url

The URL of the documentation of the project

### `package.build`
**Type:** [Build](#build)

### `package.host-dependencies`
**Type:** Object

Deprecated in favor of `package.host-dependencies`
```toml
[package.host-dependencies]
python = ">=3.8"
```

### `package.build-dependencies`
**Type:** Object

Deprecated in favor of `package.build-dependencies`

### `package.run-dependencies`
**Type:** Object

Deprecated in favor of `package.run-dependencies`

### `package.target`
**Type:** Object

Machine-specific aspects of the package
```toml
[package.target]
linux = "{'host-dependencies': {'python': '3.8'}}"
```

## platform
A supported operating system and processor architecture pair.

## py-p-i-git-branch-requirement
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

## py-p-i-git-rev-requirement
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

## py-p-i-git-tag-requirement
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

## py-p-i-options
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

## py-p-i-path-requirement
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

## py-p-i-url-requirement
### `py-p-i-url-requirement.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-url-requirement.url`
**Type:** String

A URL to a remote source or wheel

## py-p-i-version
### `py-p-i-version.extras`
**Type:** List[String]

The [PEP 508 extras](https://peps.python.org/pep-0508/#extras) of the package

### `py-p-i-version.version`
**Type:** String

The version of the package in [PEP 440](https://www.python.org/dev/peps/pep-0440/) format

### `py-p-i-version.index`
**Type:** String

The index to fetch the package from

## s3-options
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

## system-requirements
Platform-specific requirements

### `system-requirements.linux`
**Type:** Number | String

The minimum version of the Linux kernel
```toml
[system-requirements]
linux = "5.4"
```
```toml
[system-requirements]
linux = "4.19"
```

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
```toml
[system-requirements]
libc = {"family": "glibc", "version": "2.28"}
```
```toml
[system-requirements]
libc = 2.17
```

### `system-requirements.cuda`
**Type:** Number | String

The minimum version of CUDA
```toml
[system-requirements]
cuda = "12.0"
```

### `system-requirements.archspec`
**Type:** String

The architecture the project supports

### `system-requirements.macos`
**Type:** Number | String

The minimum version of MacOS
```toml
[system-requirements]
macos = "11.0"
```

## target
A machine-specific configuration of dependencies, tasks and activation.

### `target.dependencies`
**Type:** Object

The `conda` dependencies, consisting of a package name and a requirement in [MatchSpec](https://github.com/conda/conda/blob/078e7ee79381060217e1ec7f9b0e9cf80ecc8f3f/conda/models/match_spec.py) format
```toml
[target.dependencies]
python = ">=3.8"
numpy = "==1.21.0"
requests = ">=2.25, <3"
channel = "pytorch"
pytorch = "1.9.0"
cmake = "{'build': 'h5b0a8f6_0', 'version': '>=3.18'}"
```

### `target.host-dependencies`
**Type:** Object

Deprecated in favor of `package.host-dependencies`
```toml
[target.host-dependencies]
python = ">=3.8"
```

### `target.build-dependencies`
**Type:** Object

Deprecated in favor of `package.build-dependencies`

### `target.pypi-dependencies`
**Type:** Object

The PyPI dependencies for this target
```toml
[target.pypi-dependencies]
numpy = "1.21.0"
requests = ">=2.25, <3"
```

### `target.tasks`
**Type:** Object

The tasks of the target
```toml
[target.tasks]
build = "python setup.py build"
test = "{'cmd': 'pytest --verbose', 'depends-on': ['build']}"
deploy = "['build', 'test']"
```

### `target.activation`
**Type:** [Activation](#activation)

The scripts used on the activation of the project for this target
```toml
[target.activation]
scripts = "['activate.sh', 'setup.sh']"
```

## task
A precise definition of a task.

### `task.cmd`
**Type:** List[String] | String

A shell command to run the task in the limited, but cross-platform `bash`-like `deno_task_shell`. See the documentation for [supported syntax](https://pixi.sh/latest/environments/advanced_tasks/#syntax)
```toml
[task]
cmd = ["echo", "Hello, World!"]
```
```toml
[task]
cmd = ["echo Hello, World!"]
```

### `task.cwd`
**Type:** String

The working directory to run the task
```toml
[task]
cwd = "path/to/dir"
```

### `task.depends-on`
**Type:** List[String] | String

The tasks that this task depends on. Environment variables will **not** be expanded. Deprecated in favor of `depends-on` from v0.21.0 onward.

### `task.depends-on`
**Type:** List[[DependsOn](#dependson) | String] | [DependsOn](#dependson) | String

The tasks that this task depends on. Environment variables will **not** be expanded.
```toml
[task]
depends-on = ["other-task"]
```
```toml
[task]
depends-on = [{"args": ["arg1", "arg2"], "task": "other-task"}]
```
```toml
[task]
depends-on = [{"args": ["arg1", "{{ forwarded }}"], "environment": "my-env", "task": "other-task"}]
```

### `task.inputs`
**Type:** List[String]

A list of `.gitignore`-style glob patterns that should be watched for changes before this command is run. Environment variables _will_ be expanded.
```toml
[task]
inputs = ["**/*.py"]
```
```toml
[task]
inputs = ["src/**/*.js"]
```
```toml
[task]
inputs = ["src/**/*.ts"]
```
```toml
[task]
inputs = ["src/**/*.tsx"]
```

### `task.outputs`
**Type:** List[String]

A list of `.gitignore`-style glob patterns that are generated by this command. Environment variables _will_ be expanded.
```toml
[task]
outputs = ["build"]
```
```toml
[task]
outputs = ["out"]
```

### `task.env`
**Type:** Object

A map of environment variables to values, used in the task, these will be overwritten by the shell.
```toml
[task.env]
key = "value"
ARGUMENT = "value"
```

### `task.description`
**Type:** String

A short description of the task
```toml
[task]
description = "Build the project"
```

### `task.clean-env`
**Type:** Boolean

Whether to run in a clean environment, removing all environment variables except those defined in `env` and by pixi itself.
```toml
[task]
clean-env = "true"
```

### `task.args`
**Type:** List[[TaskArgs](#taskargs) | String]

The arguments to pass to the task
```toml
[task]
args = ["arg1"]
```
```toml
[task]
args = ["arg2"]
```

## task-args
The arguments of a task.

### `task-args.arg`
**Type:** String

**Required:** Yes


### `task-args.default`
**Type:** String

The default value of the argument

## workspace
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
**Type:** List[String | Url | [Channel](#channel)]

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
**Type:** Url

The URL of the homepage of the project

### `workspace.repository`
**Type:** Url

The URL of the repository of the project

### `workspace.documentation`
**Type:** Url

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

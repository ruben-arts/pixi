from pathlib import Path
import shutil
import json

from ..common import verify_cli_command


def test_build_conda_package(pixi: Path, tmp_pixi_workspace: Path, doc_pixi_projects: Path) -> None:
    """
    This one tries to build the rich example project
    """

    project = doc_pixi_projects / "pixi_build_python"
    target_dir = tmp_pixi_workspace / "project"
    shutil.copytree(project, target_dir)
    shutil.rmtree(target_dir.joinpath(".pixi"), ignore_errors=True)

    manifest_path = target_dir / "pixi.toml"

    # build it
    verify_cli_command(
        [pixi, "build", "--manifest-path", manifest_path, "--output-dir", manifest_path.parent]
    )

    # really make sure that conda package was built
    package_to_be_built = next(manifest_path.parent.glob("*.conda"))

    assert package_to_be_built.exists()


def test_build_using_rattler_build_backend(
    pixi: Path, build_data: Path, tmp_pixi_workspace: Path
) -> None:
    test_data = build_data.joinpath("rattler-build-backend")
    shutil.copytree(test_data / "pixi", tmp_pixi_workspace / "pixi")
    shutil.copyfile(
        test_data / "recipes/smokey/recipe.yaml", tmp_pixi_workspace / "pixi/recipe.yaml"
    )

    manifest_path = tmp_pixi_workspace / "pixi" / "pixi.toml"

    # Running pixi build should build the recipe.yaml
    verify_cli_command(
        [pixi, "build", "--manifest-path", manifest_path, "--output-dir", manifest_path.parent],
    )

    # really make sure that conda package was built
    package_to_be_built = next(manifest_path.parent.glob("*.conda"))

    assert "smokey" in package_to_be_built.name
    assert package_to_be_built.exists()


def test_smokey(pixi: Path, build_data: Path, tmp_pixi_workspace: Path) -> None:
    test_data = build_data.joinpath("rattler-build-backend")
    # copy the whole smokey project to the tmp_pixi_workspace
    shutil.copytree(test_data, tmp_pixi_workspace / "test_data")
    manifest_path = tmp_pixi_workspace / "test_data" / "smokey" / "pixi.toml"
    verify_cli_command(
        [
            pixi,
            "install",
            "--manifest-path",
            manifest_path,
        ]
    )

    # load the json file
    conda_meta = (
        (manifest_path.parent / ".pixi/envs/default/conda-meta").glob("smokey-*.json").__next__()
    )
    metadata = json.loads(conda_meta.read_text())

    assert metadata["name"] == "smokey"


def test_source_change_trigger_rebuild(
    pixi: Path, build_data: Path, tmp_pixi_workspace: Path
) -> None:
    project = "simple-pyproject"
    test_data = build_data.joinpath(project)

    # TODO: Setting the cache dir shouldn't be necessary!
    env = {"PIXI_CACHE_DIR": str(tmp_pixi_workspace.joinpath("pixi_cache"))}

    target_dir = tmp_pixi_workspace.joinpath(project)
    shutil.copytree(test_data, target_dir)
    manifest_path = target_dir.joinpath("pyproject.toml")

    verify_cli_command(
        [
            pixi,
            "run",
            "--manifest-path",
            manifest_path,
            "get-version",
        ],
        stdout_contains="The version of simple-pyproject is 1.0.0",
        env=env,
    )

    # Bump version from 1.0.0 to 2.0.0
    init_file = target_dir.joinpath("src", "simple_pyproject", "__init__.py")
    init_file.write_text(init_file.read_text().replace("1.0.0", "2.0.0"))

    # Since we modified the source this should trigger a rebuild and therefore report 2.0.0
    verify_cli_command(
        [
            pixi,
            "run",
            "--manifest-path",
            manifest_path,
            "get-version",
        ],
        stdout_contains="The version of simple-pyproject is 2.0.0",
        env=env,
    )


def test_editable_pyproject(pixi: Path, build_data: Path, tmp_pixi_workspace: Path) -> None:
    project = "editable-pyproject"
    test_data = build_data.joinpath(project)

    # TODO: Setting the cache dir shouldn't be necessary!
    env = {
        "PIXI_CACHE_DIR": str(tmp_pixi_workspace.joinpath("pixi_cache")),
    }

    target_dir = tmp_pixi_workspace.joinpath(project)
    shutil.copytree(test_data, target_dir)
    manifest_path = target_dir.joinpath("pyproject.toml")

    verify_cli_command(
        [
            pixi,
            "install",
            "--manifest-path",
            manifest_path,
        ],
        env=env,
    )

    # Verify that package is installed as editable
    verify_cli_command(
        [
            pixi,
            "run",
            "--manifest-path",
            manifest_path,
            "check-editable",
        ],
        env=env,
        stdout_contains="The package is installed as editable.",
    )

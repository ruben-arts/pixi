use pixi::activation::CurrentEnvVarBehavior;
use crate::common::PixiControl;

mod common;

#[tokio::test]
async fn test_normal_env_activation() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().await.unwrap();

    let project = pixi.project().unwrap();
    let default_env = project.default_environment();

    let normal_env = project.get_activated_environment_variables(
        &default_env,
        CurrentEnvVarBehavior::Exclude,
    ).await.unwrap();

    std::env::set_var("DIRTY_VAR", "Dookie");

    assert!(normal_env.get("CONDA_PREFIX").is_some());
    assert!(normal_env.get("DIRTY_VAR").is_none());
    // Display is not a pixi var, but it is passed into a clean env.
    assert!(normal_env.get("DISPLAY").is_none());
}

#[tokio::test]
async fn test_full_env_activation() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().await.unwrap();

    let project = pixi.project().unwrap();
    let default_env = project.default_environment();

    std::env::set_var("DIRTY_VAR", "Dookie");

    let full_env = project.get_activated_environment_variables(
        &default_env,
        CurrentEnvVarBehavior::Include,
    ).await.unwrap();
    assert!(full_env.get("CONDA_PREFIX").is_some());
    assert!(full_env.get("DIRTY_VAR").is_some());
    assert!(full_env.get("DISPLAY").is_some());
}

#[cfg(target_family = "unix")]
#[tokio::test]
async fn test_clean_env_activation() {
    let pixi = PixiControl::new().unwrap();
    pixi.init().await.unwrap();

    let project = pixi.project().unwrap();
    let default_env = project.default_environment();

    std::env::set_var("DIRTY_VAR", "Dookie");

    let clean_env = project.get_activated_environment_variables(
        &default_env,
        CurrentEnvVarBehavior::Clean,
    ).await.unwrap();
    assert!(clean_env.get("CONDA_PREFIX").is_some());
    assert!(clean_env.get("DIRTY_VAR").is_none());

    // Display is not a pixi var, but it is passed into a clean env.
    assert!(clean_env.get("DISPLAY").is_some());
}



use std::fs;
use eyre::Result;
use tempfile::tempdir;

use magnet::configs::ManifestConfig;

#[test]
fn test_magnet_config_create_and_parse() -> Result<()> {
    // Create a temporary directory for our test
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("Magnet.toml");
    
    // Create a basic configuration
    let mut config = ManifestConfig::new();
    // Create package section
    config.package = Some(magnet::configs::PackageConfig {
        name: "test-project".to_string(),
        version: "0.1.0".to_string(),
        ..Default::default()
    });
    // Create workspace section
    config.workspace = Some(magnet::configs::WorkspaceConfig {
        members: vec!["crates/*".to_string()],
        ..Default::default()
    });
    config.dependencies.insert("serde".to_string(), "1.0.0".into());
    
    // Write the config to file
    config.save_to_file(&config_path)?;
    
    // Read it back
    let read_config = ManifestConfig::from_file(&config_path)?;
    
    // Verify it matches what we wrote
    assert!(read_config.package.is_some());
    if let Some(package) = &read_config.package {
        assert_eq!(package.name, "test-project".to_string());
        assert_eq!(package.version, "0.1.0".to_string());
    }
    assert!(read_config.workspace.is_some());
    if let Some(workspace) = &read_config.workspace {
        assert_eq!(workspace.members, vec!["crates/*".to_string()]);
    }
    assert!(read_config.dependencies.contains_key("serde"));
    
    Ok(())
}

#[test]
fn test_workspace_crate_detection() -> Result<()> {
    // Create a temporary directory for our test
    let temp_dir = tempdir()?;
    let workspace_dir = temp_dir.path().join("workspace");
    fs::create_dir_all(&workspace_dir)?;
    
    // Create a Magnet.toml file
    let config_path = workspace_dir.join("Magnet.toml");
    let mut config = ManifestConfig::new();
    config.package = Some(magnet::configs::PackageConfig {
        name: "test-workspace".to_string(),
        version: "0.1.0".to_string(),
        ..Default::default()
    });
    config.workspace = Some(magnet::configs::WorkspaceConfig {
        members: vec!["crates/*".to_string()],
        ..Default::default()
    });
    config.save_to_file(&config_path)?;
    
    // Create some test crates
    let crates_dir = workspace_dir.join("crates");
    fs::create_dir_all(&crates_dir)?;
    
    // Create crate1
    let crate1_dir = crates_dir.join("crate1");
    fs::create_dir_all(&crate1_dir)?;
    fs::write(
        crate1_dir.join("Cargo.toml"),
        r#"[package]
name = "crate1"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
    )?;
    
    // Create crate2
    let crate2_dir = crates_dir.join("crate2");
    fs::create_dir_all(&crate2_dir)?;
    fs::write(
        crate2_dir.join("Cargo.toml"),
        r#"[package]
name = "crate2"
version = "0.1.0"
edition = "2024"

[dependencies]
crate1 = { version = "0.1.0" }
"#,
    )?;
    
    // Now test that we can detect these crates
    use magnet::models::WorkspaceModel;
    
    // Create a workspace model manually since we don't have a real workspace manager
    let workspace = WorkspaceModel {
        name: "test-workspace".to_string(),
        description: Some("Test workspace".to_string()),
        members: vec!["crates/*".to_string()],
        exclude: vec![],
        resolver: Some("2".to_string()),
        paths: Default::default(),
        custom: Default::default(),
        dependencies: Default::default(),
        patch: None,
        root_path: workspace_dir.clone(),
        source_path: workspace_dir.join("Magnet.toml"),
    };
    
    let crates = workspace.list_packages()?;
    
    // We should have found 2 crates
    assert_eq!(crates.len(), 2);
    
    // Make sure we found both crates by name
    let crate_names: Vec<String> = crates.iter().map(|c| c.name.clone()).collect();
    assert!(crate_names.contains(&"crate1".to_string()));
    assert!(crate_names.contains(&"crate2".to_string()));
    
    Ok(())
}
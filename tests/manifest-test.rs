use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;
use thatproject::manifest::Manifest;

static TEST_LOCK: Mutex<()> = Mutex::new(());

fn with_temp_manifest<F>(test_fn: F)
where
  F: FnOnce(),
{
  let _guard = TEST_LOCK.lock().unwrap();
  let temp_dir = TempDir::new().unwrap();
  
  thatproject::config::set_project_dir(temp_dir.path().to_str().unwrap().to_string());
  thatproject::config::set_manifest_filename("manifest.json".to_string());
  
  test_fn();
  
  thatproject::config::reset_project_dir();
  thatproject::config::reset_manifest_filename();
}

// #[test]
// fn test_create_manifest() {
//   with_temp_manifest(|| {
//     let result = Manifest::create("test-project".to_string(), "A test project".to_string());
//     assert!(result.is_ok(), "Failed to create manifest");

//     let content = fs::read_to_string(thatproject::config::get_manifest_path()).unwrap();
//     assert!(content.contains("test-project"));
//     assert!(content.contains("A test project"));
//   });
// }

// #[test]
// fn test_load_manifest() {
//   with_temp_manifest(|| {
//     Manifest::create("test-project".to_string(), "A test project".to_string()).unwrap();

//     let loaded = Manifest::load();
//     assert!(loaded.is_ok(), "Failed to load manifest");

//     let manifest = loaded.unwrap();
//     assert_eq!(manifest.name, "test-project");
//     assert_eq!(manifest.description, "A test project");
//     assert!(manifest.sources.is_empty());
//   });
// }

// #[test]
// fn test_add_source() {
//   with_temp_manifest(|| {
//     Manifest::create("test-project".to_string(), "A test project".to_string()).unwrap();
//     Manifest::add_source("source1".to_string()).unwrap();

//     let manifest = Manifest::load().unwrap();
//     assert_eq!(manifest.sources.len(), 1);
//     assert_eq!(manifest.sources[0], "source1");
//   });
// }

// #[test]
// fn test_add_multiple_sources() {
//   with_temp_manifest(|| {
//     Manifest::create("test-project".to_string(), "A test project".to_string()).unwrap();
//     Manifest::add_source("source1".to_string()).unwrap();
//     Manifest::add_source("source2".to_string()).unwrap();
//     Manifest::add_source("source3".to_string()).unwrap();

//     let manifest = Manifest::load().unwrap();
//     assert_eq!(manifest.sources.len(), 3);
//     assert!(manifest.sources.contains(&"source1".to_string()));
//     assert!(manifest.sources.contains(&"source2".to_string()));
//     assert!(manifest.sources.contains(&"source3".to_string()));
//   });
// }

// #[test]
// fn test_add_duplicate_source_is_idempotent() {
//   with_temp_manifest(|| {
//     Manifest::create("test-project".to_string(), "A test project".to_string()).unwrap();
//     Manifest::add_source("source1".to_string()).unwrap();
//     Manifest::add_source("source1".to_string()).unwrap();

//     let manifest = Manifest::load().unwrap();
//     assert_eq!(manifest.sources.len(), 1);
//     assert_eq!(manifest.sources[0], "source1");
//   });
// }

// #[test]
// fn test_load_nonexistent_manifest() {
//   with_temp_manifest(|| {
//     let result = Manifest::load();
//     assert!(result.is_err(), "Expected load to fail for nonexistent manifest");
//   });
// }

// #[test]
// fn test_add_source_to_nonexistent_manifest() {
//   with_temp_manifest(|| {
//     let result = Manifest::add_source("source1".to_string());
//     assert!(result.is_err(), "Expected add_source to fail for nonexistent manifest");
//   });
// }

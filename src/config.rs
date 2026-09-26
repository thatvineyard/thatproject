use std::cell::RefCell;

// DEFAULTS

const DEFAULT_MANIFEST_FILENAME: &str = "manifest.json";
const DEFAULT_PROJECT_DIR: &str = ".thatproject";

// PROJECT DIR

thread_local! {
  static PROJECT_DIR: RefCell<String> = RefCell::new(DEFAULT_PROJECT_DIR.to_string());
}

pub fn set_project_dir(dir: String) {
  PROJECT_DIR.with(|d| {
    *d.borrow_mut() = dir;
  })
}

pub fn get_project_dir() -> String {
  PROJECT_DIR.with(|d| d.borrow().clone())
}

pub fn reset_project_dir() {
  PROJECT_DIR.with(|f| {
    *f.borrow_mut() = DEFAULT_PROJECT_DIR.to_string();
  })
}

// MANIFEST FILENAME

thread_local! {
  static MANIFEST_FILENAME: RefCell<String> = RefCell::new(DEFAULT_MANIFEST_FILENAME.to_string());
}

pub fn set_manifest_filename(filename: String) {
  MANIFEST_FILENAME.with(|f| {
    *f.borrow_mut() = filename;
  })
}

fn get_filename() -> String {
  MANIFEST_FILENAME.with(|f| f.borrow().clone())
}

pub fn reset_manifest_filename() {
  MANIFEST_FILENAME.with(|f| {
    *f.borrow_mut() = DEFAULT_MANIFEST_FILENAME.to_string();
  })
}

// MANIFEST PATH

pub fn get_manifest_path() -> String {
  format!("{}/{}",  get_project_dir(), get_filename())
}

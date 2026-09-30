use std::cell::RefCell;

const PROJECT_DIR: &str = ".thatproject";

// DEFAULTS

const DEFAULT_MANIFEST_FILENAME: &str = "manifest.json";
const DEFAULT_CONTEXT_DIR: &str = ".";

// CONTEXT DIR

thread_local! {
  static CONTEXT_DIR: RefCell<String> = RefCell::new(DEFAULT_CONTEXT_DIR.to_string());
}

pub fn set_context_dir(dir: String) {
    CONTEXT_DIR.with(|d| {
        *d.borrow_mut() = dir;
    })
}

pub fn get_context_dir() -> String {
    CONTEXT_DIR.with(|d| d.borrow().clone())
}

pub fn reset_context_dir() {
    CONTEXT_DIR.with(|f| {
        *f.borrow_mut() = DEFAULT_CONTEXT_DIR.to_string();
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

// THATPROJECT DIR

pub fn get_thatproject_dir() -> String {
    format!("{}/{}", get_context_dir(), PROJECT_DIR)
}

// MANIFEST PATH

pub fn get_manifest_path() -> String {
    format!("{}/{}", get_thatproject_dir(), get_filename())
}

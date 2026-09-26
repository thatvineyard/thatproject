use crate::app_context::AppContext;

pub fn run(_context: &AppContext, name: String, description: String, task_dir: String) {
  let _ = crate::features::initialization::create_manifest(name, description, task_dir);
}
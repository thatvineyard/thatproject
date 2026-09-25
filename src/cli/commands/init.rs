pub fn run(name: String, description: String) {
  let _ = crate::features::initialization::create_manifest(name, description);
}
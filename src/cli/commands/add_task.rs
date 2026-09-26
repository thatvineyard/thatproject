pub fn run(name: String, description: String) {
  if let Err(err) = crate::features::tasks::create_task(name, description) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
  }
}
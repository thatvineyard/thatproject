pub fn slugify(name: &str) -> String {
  let slug = name
    .chars()
    .flat_map(char::to_lowercase)
    .fold(String::new(), |mut slug, ch| {
      if ch.is_alphanumeric() {
        slug.push(ch);
      } else if !slug.is_empty() && !slug.ends_with('-') {
        slug.push('-');
      }
      slug
    });
  
  let slug = slug.trim_matches('-');
  if slug.is_empty()  {
    "task".to_owned()
  } else {
    slug.to_owned()
  }
}
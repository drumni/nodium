use std::path::{Path, PathBuf};

pub struct Metadata {
    pub name: String,
    pub tags: Vec<String>,
}

pub fn get_metadata(path: PathBuf, root_path: PathBuf) -> Metadata {
    let path = Path::new(&path);
    // split every path into its components using regex and other methods
    // than add all items to the list and remove duplicates

    // split path in all regments att "/" and "\" and "."
    let path_string = path.to_str().unwrap();
    let mut tags: Vec<String> = path_string
        .split(|c| c == '/' || c == '\\' || c == '.')
        .map(|s| s.to_string())
        .collect();
    // filter tags out which contain in root_path
    let root_path_string = root_path.to_str().unwrap();
    tags = tags
        .iter()
        .filter(|s| !s.contains(root_path_string))
        .map(|s| s.to_string())
        .collect();
    
    tags.retain(|s| !s.is_empty());
    tags.dedup();
    tags.reverse();
    // wrap all tags in [[]]
    tags = tags.iter().map(|s| format!("[[{}]]", s)).collect();

    let name = match path.file_name() {
        Some(f) => f.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    Metadata {
        name,
        tags,
    }
}

pub fn get_metadata_string(path: PathBuf, root_path: PathBuf) -> String {
    let metadata = get_metadata(path, root_path);
    format!(
        "---\nname: {}\nlinks: {:?}\n---\n",
        metadata.name, metadata.tags
    )
}

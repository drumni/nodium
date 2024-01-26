// utils.rs
use ignore::WalkBuilder;
use std::{ffi::OsStr, path::{Path, PathBuf}};

pub fn open_folder_dialog(app: tauri::AppHandle) {
    tauri::api::dialog::FileDialogBuilder::new().pick_folder(|folder| {
        let path = serde_json::to_string(&folder).unwrap();
        tauri::async_runtime::spawn(async move {
            // convert windows path to unix path
            let path = path.replace("//", "\\");
            tauri::Manager::emit_all(&app, "folder-selected", Some(path)).unwrap();
        });
    });
}

pub fn load_folder(path: String) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    // check if directory exists
    let path = Path::new(&path);
    if !path.is_dir() {
        return items;
    }

    let paths = match std::fs::read_dir(path.clone()) {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error encountered while reading the directory {:?}: {:?}", path, e);
            return items;
        }
    };

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        let path = path.replace("\\", "/");
        items.push(path);
    }
    items
}

pub fn get_item_count(path: String) -> i32 {
    match std::fs::read_dir(path) {
        Ok(paths) => paths.count().try_into().unwrap(),
        Err(_) => -1,
    }
}

pub fn get_total_count(path: String) -> i32 {
    let mut total_count = 0;

    // Ensure the path points to a directory
    let path = Path::new(&path);
    if !path.is_dir() {
        return -1;
    }

    let walker = WalkBuilder::new(path)
        .hidden(false)
        .follow_links(false)
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = Path::new(entry.path().to_str().unwrap());

                if path.is_file() {
                    total_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Error encountered while walking the path: {:?}", e);
            }
        }
    }

    total_count
}

pub fn load_gitignore(path: String) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    // read the file
    let contents = std::fs::read_to_string(path).expect("Please add Gitignore file to the root of the project");
    // split the file into lines
    let lines = contents.split("\n");
    // iterate over the lines
    for line in lines {
        // if the line is not empty and not a comment
        if line != "" && !line.starts_with("#") {
            // add the line to the items
            items.push(line.to_string());
        }
    }
    items
}

pub fn check_markdown(root_path_string: String, file_path_string: String) -> bool {
    let default_vault_dir = crate::config::get_or_default("DEFAULT_VAULT_DIR", "docs");
    // let vault_path = format!("{}/files/{}{}", root, default_vault_dir, path);
    // check if it has extension
    let has_extension = Path::new(&file_path_string).extension().is_some();
    let vault_file_path = format!(
        "{}/{}/{}{}",
        root_path_string,
        default_vault_dir,
        if has_extension { "files" } else { "folder" },
        file_path_string
    );
    let file_path = format!("{}{}", root_path_string, file_path_string);
    let path = Path::new(&vault_file_path);
    let md_path = path.with_extension("md");

    // create new empty file
    // create directory if it doesn't exist
    let parent = match md_path.parent() {
        Some(p) => p,
        None => {
            eprintln!("Error encountered while getting the parent directory of the path");
            return false;
        }
    };
    if !parent.exists() {
        match std::fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "Error encountered while creating the parent directory: {:?}",
                    e
                );
                return false;
            }
        }
    }

    let file_path = Path::new(&file_path).to_path_buf();
    let file = std::fs::File::create(md_path.clone());
    match file {
        Ok(_) => {
            generate_markdown(md_path, file_path)
        }
        Err(e) => {
            eprintln!("Error encountered while creating the file: {:?}", e);
            false
        }
    }
}

pub fn generate_markdown(md_path: PathBuf, file_path: PathBuf) -> bool {
    let extension: String = match file_path.extension().and_then(OsStr::to_str) {
        Some(e) => e.to_string(),
        None => "".to_string(),
    };

    let metadata = crate::metadata::get_metadata_string(file_path.clone());
    let mut markdown_lines: Vec<String> = Vec::new();
    let file_name = match file_path.file_name() {
        Some(f) => f.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    // check if text is readable and if it has more than 420 characters then truncate it
    let text = match file_path.is_file() {
        true => match std::fs::read_to_string(file_path.clone()) {
            Ok(t) => {
                if t.len() > 420 {
                    format!("\n > It's a heavy file, so we are showing only the first 420 characters of the file.\n\n
                    {}...", &t[..420])
                } else {
                    t
                }
            }
            Err(_) => {
                format!("[{}]({})", file_name, file_path.clone().into_os_string().into_string().unwrap())
            }
        },
        false => format!("[{}]({})", file_name, file_path.clone().into_os_string().into_string().unwrap())
    };

    markdown_lines.push(metadata);
    markdown_lines.push(format!("# Documentation for {}", file_name));
    markdown_lines.push(format!("```{}", extension));
    markdown_lines.push(text);
    markdown_lines.push("```".to_string());

    let markdown = markdown_lines.join("\n");

    match std::fs::write(md_path, markdown) {
        Ok(_) => return true,
        Err(e) => {
            eprintln!("Error encountered while writing to the file: {:?}", e);
            return false;
        }
    }
}

// Analyze markdown file
pub fn analyze_markdown(path: String) -> bool {
    let path = Path::new(&path);
    let extension = path.extension().and_then(OsStr::to_str);
    // println!("Generating Markdown For file: {:?}", path);
    match extension {
        Some("md") => true,
        _ => false,
    }
}

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

mod file_utils;
use file_utils::process_file;

fn main() {
    let second_brain_path = env::var("secondbrain").expect("Set the SECOND_BRAIN_PATH variable");
    let public_folder_path_copy = env::var("public_secondbrain").expect("Set the PUBLIC_FOLDER_PATH_COPY variable");
    // let public_brain_image_path = format!("{}/{}", public_folder_path_copy, "images");
    let public_brain_image_path = env::var("public_secondbrain").expect("Set the PUBLIC_FOLDER_PATH_COPY variable");

    let mut images_map: HashMap<String, PathBuf> = HashMap::new();
    build_images_map(Path::new(&second_brain_path), &mut images_map).unwrap();

    // visit_dirs(Path::new(&second_brain_path), &public_folder_path_copy, &public_brain_image_path, &images_map).unwrap();
    match visit_dirs(Path::new(&second_brain_path), &public_folder_path_copy, &public_brain_image_path, &images_map) {
        Ok(_) => (),
        Err(e) => println!("An error occurred: {}", e),
    }

}

fn visit_dirs(dir: &Path, public_folder: &str, public_brain_image_path: &str, images_map: &HashMap<String, PathBuf>) -> std::io::Result<()> {
    // println!("Visiting directory: {}", dir.display());
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, public_folder, public_brain_image_path, images_map)?;
            } else {
                ////DEBUG:
                ////skip if file is not "Folder Structure PARA.md"
                //if let Some(file_name) = path.file_name() {
                //    if file_name != "Folder Structure PARA.md" {
                //        continue;
                //    }
                //}


                if let Some(extension) = path.extension() {
                    if extension == "md" {
                        process_file(&path, public_folder, public_brain_image_path, images_map)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn build_images_map(dir: &Path, images_map: &mut HashMap<String, PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                build_images_map(&path, images_map)?;
            } else {
                if let Some(extension) = path.extension() {
                    if extension == "png" || extension == "jpg" || extension == "gif" {
                        if let Some(file_name) = path.file_name() {
                            images_map.insert(file_name.to_str().unwrap().to_string(), path.clone());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}


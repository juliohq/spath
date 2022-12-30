use std::{env, fs::{self, DirEntry}, path::Path, io::Error, cmp::Ordering};

fn main() {
    // Parse arguments
    let args = env::args().collect::<Vec<String>>();
    let root_path = args.iter().skip(1).next();
    let dry_run = args.contains(&"-d".to_string());
    
    if dry_run {
        println!("Dry run mode, it will do nothing");
    }
    
    // Run
    if root_path.is_some() {
        let path = root_path.unwrap();
        
        if Path::new(path).exists() {
            scan(path, None, dry_run);
        } else {
            println!("'{path}' does not exist");
        }
    } else {
        println!("No path specified, exiting...");
    }
}

fn scan(path: &String, parent: Option<&String>, dry_run: bool) {
    println!("Scanning '{path}'");
    
    let paths = fs::read_dir(path).unwrap();
    let mut entries = paths.into_iter().collect::<Vec<Result<DirEntry, Error>>>();
    
    // Sort empty folders first
    entries.sort_by(sort);
    
    // Check if current folder is empty and remove it
    if entries.is_empty() {
        if dry_run {
            println!("Folder '{}' is empty", path);
        } else {
            println!("Folder '{}' is empty, removing...", path);
            
            match fs::remove_dir(path) {
                Ok(_) => {
                    println!("'{}' removed", path);
                },
                Err(e) => {
                    panic!("Error {:?}", e);
                }
            }
        }
    } else {
        // Iterate for paths in the current directory
        for entry in entries {
            if entry.is_ok() {
                let p = entry.unwrap().path();
                
                if p.is_dir() {
                    if parent.is_some() {
                        println!("Folder '{}' found", p.display());
                    } else {
                        println!("'{}' is the root folder", path);
                    }
                    
                    let x = &p.to_str().unwrap().to_string();
                    scan(x, Some(path), dry_run);
                } else {
                    println!("File '{}' found", p.display());
                    
                    // Move contents of the current folder to the parent one
                    if parent.is_some() {
                        let from = p.to_str().unwrap();
                        let to = &Path::new(parent.unwrap()).join(p.file_name().unwrap());
                        
                        if dry_run {
                            println!("Skip renaming '{}' to '{}'", p.to_str().unwrap(), to.to_str().unwrap());
                        } else {
                            match fs::rename(from, to) {
                                Ok(_) => {},
                                Err(e) => {
                                    panic!("Error {:?}", e);
                                }
                            }
                            
                            println!("Renamed '{}' to '{}'", p.to_str().unwrap(), to.to_str().unwrap());
                        }
                    }
                }
            } else {
                panic!("Error {:?}", entry.unwrap());
            }
        }
        
        // Remove directory
        let p = Path::new(path);
        
        if p.is_dir() && p.exists() && parent.is_some() {
            if dry_run {
                println!("Skip removing {}", path);
            } else {
                match fs::remove_dir(p) {
                    Ok(_) => {
                        println!("Removed {}", path);
                    },
                    Err(e) => {
                        println!("Error {:?}", e);
                    }
                }
            }
        }
    }
}

fn sort<'a, 'b>(a: &'a Result<DirEntry, Error>, b: &'b Result<DirEntry, Error>) -> Ordering {
    let path_a = a.as_ref().unwrap().path();
    let path_b = b.as_ref().unwrap().path();
    
    if path_a.is_dir() && path_b.is_dir() {
        if fs::read_dir(path_a).unwrap().into_iter().collect::<Vec<Result<DirEntry, Error>>>().is_empty() {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
    
    Ordering::Equal
}
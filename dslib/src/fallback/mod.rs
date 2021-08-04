use std::path::PathBuf;
use std::sync::{Mutex};
use std::collections::HashMap;
use std::fs::{read_dir, metadata};
use lazy_static::*;
use super::Directory;

//Lazy static to store cached scan info
lazy_static! {
    static ref SCAN_CACHE: Mutex<Option<FileInternal>> = Mutex::new(None);
}

pub fn verify(_dir: &std::path::Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(true)
}

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    *SCAN_CACHE.lock().unwrap() = Some(FileInternal::new(dir)?);
    Ok(())
}

pub fn query(dir: PathBuf) -> Result<Option<Directory>, Box<dyn ::std::error::Error>> {
    todo!();
}

#[derive(Debug)]
pub struct FileInternal {
    pub path: PathBuf,
    pub size: usize,
    pub children: Option<HashMap<PathBuf,FileInternal>>,
}

impl FileInternal {
    //Constructor to initialise a FileInternal and recursively scan through all its children
    pub fn new(path: PathBuf) -> Result<FileInternal, std::io::Error> {
        //Get useful metadata for the given path
        let meta = metadata(&path)?;
        //If this is a file we can just return the path and the size wrapped into a FileInternal
        if meta.is_file() {
            trace!("{}: {} bytes", path.to_str().expect("Tell Ben his trace statement errors - Ben"), meta.len());
            return Ok(FileInternal { path: path, size: meta.len() as usize, children: None });
        }

        //Instantiate Empty Struct
        let mut this_folder = FileInternal { path: path.clone(), size: 0, children: None };
        let mut this_folder_children: HashMap<PathBuf, FileInternal> = HashMap::new();
        let dir_info = read_dir(path)?;
        for child in dir_info {
            let child_path = child?.path();
            let child_data: FileInternal = FileInternal::new(child_path.clone())?;  //Run this function to get the information for the child
            this_folder.size += child_data.size;                                    //Append the size of the child to this_folder
            this_folder_children.insert(child_path, child_data);            //Add this to the children
        }
        this_folder.children = Some(this_folder_children);
        Ok(this_folder)
    }
}

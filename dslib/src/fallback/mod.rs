use std::path::{PathBuf, Path};
use std::sync::{Mutex};
use std::collections::HashMap;
use std::fs::{read_dir, symlink_metadata};
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::io::{Error, ErrorKind};
use lazy_static::*;
use super::{Directory, File};


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
    let scandata = SCAN_CACHE.lock().unwrap();
    let pathstart = scandata.as_ref().unwrap().path.clone();
    //Strip the path to the scanned directory off the path, making the query path relative to the start of the scan not the root directory
    let truncated_query_path = dir.strip_prefix(&scandata.as_ref().unwrap().path.clone());
    let mut currentfile: &FileInternal = scandata.as_ref().unwrap();
    for segment in truncated_query_path {
        match &currentfile.children.as_ref() {
            Some(cf) => {
                let seg_buf = &segment.to_path_buf();
                if cf.contains_key(seg_buf) {
                    currentfile = &cf[seg_buf];
                }
                else {
                    return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, format!("{:?} does not exist in scan records!", &currentfile.path))))
                }
            },
            None => return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, format!("{:?} is not a directory! Cannot get its children!", &currentfile.path))))
        }
    }
    //If we ended up with a file something is wrong
    if currentfile.children.is_none() {
        return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, format!("{:?} is not a directory! Cannot get its children!", &currentfile.path))))
    }
    Ok(Some(currentfile.to_directory()?))  
}

#[derive(Debug)]
pub struct FileInternal {
    pub path: PathBuf,
    pub size: usize,
    pub children: Option<HashMap<PathBuf,FileInternal>>,
}

impl FileInternal {
    ///Constructor to initialise a FileInternal and recursively scan through all its children
    pub fn new(path: PathBuf) -> Result<FileInternal, std::io::Error> {
        debug!("Scanning: {:?}:", path);
        //Get useful metadata for the given path
        let meta = symlink_metadata(&path)?;
        //If this is a file we can just return the path and the size wrapped into a FileInternal
        //Also do this if it is a simlink since we dont' want to follow those to avoid recording the same file twice
        if meta.is_file()|| meta.file_type().is_symlink() {
            trace!("{:?}: {} bytes", path, meta.len());
            return Ok(FileInternal { path: path, size: meta.len() as usize, children: None });
        }
        #[cfg(unix)]
        if meta.file_type().is_socket() {
            trace!("{:?}: {} bytes", path, meta.len());
            return Ok(FileInternal { path: path, size: meta.len() as usize, children: None });
        }

        //Instantiate empty struct for this folder
        let mut this_folder = FileInternal { path: path.clone(), size: 0, children: None };
        let mut this_folder_children: HashMap<PathBuf, FileInternal> = HashMap::new();
        let dir_info = read_dir(path)?;
        for child in dir_info {
            let child_path = child?.path();
            let child_data: FileInternal = FileInternal::new(child_path.clone())?;  //Run this function to get the information for the child
            this_folder.size += child_data.size;                                    //Append the size of the child to this_folder
            this_folder_children.insert(child_path, child_data);                    //Add this to the children
        }
        this_folder.children = Some(this_folder_children);
        Ok(this_folder)
    }

    ///Converts this into a directory, preserves only immediate children!
    pub fn to_directory(&self) -> Result<Directory, Box<dyn ::std::error::Error>> {
        match &self.children {
            Some(_) => {
                let mut stripped_self: Directory = Directory { path: self.path.clone(), size: self.size, files: Vec::new() };
                for child in self.children.as_ref().unwrap().values() {
                    stripped_self.files.push(File { path: child.path.clone(), size: child.size, directory: child.children.is_some() });
                }
                Ok(stripped_self)
            },
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("{:?} is not a directory! Cannot get its children!", self.path)))),
        }
    }
}

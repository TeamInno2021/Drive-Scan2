use std::hash::Hash;
use std::path::{PathBuf, Path};
use std::sync::{Mutex};
use std::collections::HashMap;
use std::fs::{Metadata, read_dir, symlink_metadata};
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::io::{Error, ErrorKind};
use lazy_static::*;
use super::{File};


//Lazy static to store cached scan info
lazy_static! {
    static ref SCAN_CACHE: Mutex<Option<HashFile>> = Mutex::new(None);
}

pub fn verify(_dir: &std::path::Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(true)
}

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    info!("Beginning scan of {:?}",dir);
    *SCAN_CACHE.lock().unwrap() = Some(HashFile::init(dir)?);
    Ok(())
}

pub fn query(dir: PathBuf) -> Result<Option<File>, Box<dyn ::std::error::Error>> {
    let scandata = SCAN_CACHE.lock().unwrap();
    let pathstart = &scandata.as_ref().unwrap().path.clone();
    //Strip the path to the scanned directory off the path, making the query path relative to the start of the scan not the root directory
    let truncated_query_path = dir.strip_prefix(pathstart)?;
    let mut currentfile: &HashFile = scandata.as_ref().unwrap();
    for segment in truncated_query_path {
        match currentfile.children.as_ref() {
            Some(cf) => {
                let seg_buf = &PathBuf::from(segment);
                if cf.contains_key(seg_buf) {
                    currentfile = &cf[seg_buf];
                }
                else {
                    return Err(Box::new(Error::new(ErrorKind::NotFound, format!("{:?} does not exist in scan records!", &currentfile.path))))
                }
            },
            None => return Err(Box::new(Error::new(ErrorKind::NotFound, format!("{:?} is not a directory! Cannot get its children!", &currentfile.path))))
        }
    }
    //If we ended up with a file something is wrong
    if currentfile.children.is_none() {
        return Err(Box::new(Error::new(ErrorKind::NotFound, format!("{:?} is not a directory! Cannot get its children!", &currentfile.path))))
    }
    Ok(Some(currentfile.to_file()))  
}

#[derive(Debug)]
//Simmillar to dslib::File
pub struct HashFile {
    pub path: PathBuf,
    pub size: usize,
    pub children: Option<HashMap<PathBuf,HashFile>>,
}

impl HashFile {
    ///Constructor to initialise a FileInternal and recursively scan through all its children
    pub fn init(path: PathBuf) -> Result<HashFile, Error> {
        trace!("Scanning: {:?}:", path);
        //Get useful metadata for the given path
        let meta_res = symlink_metadata(&path);
        //Assume not a directory and a file size of zero if we don't have perms to metadata
        if meta_res.is_err() && meta_res.as_ref().err().unwrap().kind() == ErrorKind::PermissionDenied {
            debug!("Unable to query {:?} for metadata: Permission Denied!\nassuming path is not a directory and has a size of 0", path);
            return Ok(HashFile { path: path, size: 0, children: None })
        }
        else if meta_res.is_err() && meta_res.as_ref().err().unwrap().kind() == ErrorKind::Other {
            warn!("Unable to query {:?} for metadata: Unknown Error: \"{}\"!\nAssuming path is not a directory and has a size of 0", path, meta_res.as_ref().err().unwrap().to_string());
            return Ok(HashFile { path: path, size: 0, children: None })
        }
        #[cfg(windows)] {
            if meta_res.is_err() && meta_res.as_ref().err().unwrap().raw_os_error() == Some(0x20) {
                debug!("Unable to query {:?} for metadata: Permission Denied!\nassuming path is not a directory and has a size of 0", path);
                return Ok(HashFile { path: path, size: 0, children: None })
            }
        }   
        let meta: Metadata = meta_res?;
        
        //Return the path and size with no children if the file is not a normal directory
        if meta.is_file()|| meta.file_type().is_symlink() {
            trace!("{:?}: {} bytes", path, meta.len());
            return Ok(HashFile { path: path, size: meta.len() as usize, children: None });
        }
        #[cfg(unix)] {
            let filetype = meta.file_type();
            if filetype.is_socket() || filetype.is_fifo() || filetype.is_block_device() || filetype.is_char_device() {
                trace!("{:?}: {} bytes", path, meta.len());
                return Ok(HashFile { path: path, size: meta.len() as usize, children: None });
            }
        }   

        //Instantiate empty struct for this folder
        let mut this_folder = HashFile { path: path.clone(), size: 0, children: None };
        let mut this_folder_children: HashMap<PathBuf, HashFile> = HashMap::new();
        let dir_info_res = read_dir(path.clone());
        if dir_info_res.is_err() && dir_info_res.as_ref().err().unwrap().kind() == ErrorKind::PermissionDenied {
            debug!("Unable to get the children of {:?}: Permission Denied!\nAssuming path has no children and a size of 0", path);
            return Ok(HashFile { path: path, size: 0, children: Some(HashMap::new()) })
        }
        else if dir_info_res.is_err() && dir_info_res.as_ref().err().unwrap().kind() == ErrorKind::Other {
            warn!("Unable to get the children of {:?}: Unknown Error: \"{}\"\nAssuming path has no children and a size of 0", path, dir_info_res.as_ref().err().unwrap().to_string());
            return Ok(HashFile { path: path, size: 0, children: Some(HashMap::new()) })
        }
        let dir_info = dir_info_res?;
        for child in dir_info {
            let child_path = child?.path();
            let child_data: HashFile = HashFile::init(child_path.clone())?;                                 //Run this function to get the information for the child
            this_folder.size += child_data.size;                                                            //Append the size of the child to this_folder
            this_folder_children.insert(PathBuf::from(child_path.file_name().unwrap()), child_data);   //Add this to the children
        }
        this_folder.children = Some(this_folder_children);
        trace!("{:?}: {} bytes", this_folder.path, this_folder.size);
        Ok(this_folder)
    }

    ///Converts the object, and its children to a `dslib::File`. 
    ///The `children` property of all children of the output file will be set to:
    ///- `None`: The file is not a directory
    ///- `Some(vec![File;0])`: The file is a directory
    pub fn to_file(&self) -> File {       
        let mut cloned_self: File = File { path: self.path.clone(), size: self.size, children: None };
        if self.children.is_some() {
            let mut trimmed_file_children = Vec::new();
            for child in self.children.as_ref().unwrap().values() {
                trimmed_file_children.push(File { path: child.path.clone(), size: child.size, children: child.children.as_ref().map(|_| Vec::new())});
            }
            cloned_self.children = Some(trimmed_file_children);
        }
        cloned_self
    }   
}

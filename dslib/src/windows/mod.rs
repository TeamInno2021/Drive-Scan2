mod alt;
mod data;
mod drive;
mod error;
mod filesystem;
mod mft;
mod winapi;

use super::File;
use drive::DriveInfo;
use error::OsError;
use std::mem::size_of;
use std::path::{Component, Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    // Make sure we aren't running on a 32-bit system (just in case)
    // This means we can enforce that `usize` is a 64 bit integer
    if size_of::<usize>() != size_of::<u64>() {
        Ok(false)
    } else if !dir.exists() {
        Err("target path does not exist".into())
    } else if !dir.is_dir() {
        Err("target path is not a valid directory".into())
    } else {
        // The alt scanner can run on any windows target
        Ok(true)
    }
}

pub fn scan(dir: PathBuf) -> Result<(), Box<dyn ::std::error::Error>> {
    #[cfg(not(feature = "use-winalt"))]
    if let Ok(fs) = filesystem::identify(&dir) {
        if fs == "NTFS" {
            let drive = DriveInfo::parse(dir.clone())?;
            let _nodes = mft::process(drive)?;
            return Ok(());
        }
    }

    alt::scan(dir);
    Ok(())
}

pub fn query(dir: PathBuf) -> Result<Option<File>, Box<dyn ::std::error::Error>> {
    if let Some(file) = data::fetch().lock().unwrap().take() {
        let mut f = &file;

        if let Ok(relative) = dir.strip_prefix(&f.path) {
            info!("Root: {:?}", f.path);
            info!("Target: {:?}", dir);
            info!("Parsed relative path: {:?}", relative);

            for segment in relative.components() {
                if let Component::Normal(component) = segment {
                    if let Some(children) = &f.children {
                        for child in children {
                            if child.path == f.path.join(component) {
                                f = child;
                            }
                        }
                    }
                }
            }
        }

        // Note that we manually reconstruct the object instead of cloning it
        //      because we do not want to re-allocate the entire tree in memory
        let file = File {
            size: f.size,
            path: f.path.clone(),
            // Clear second layer children
            children: Some(
                // "I want a comment that says 'James is a web developer' and nothing else - James, after fixing the problem
                f.children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|mut child| {
                        child.children = child.children.map(|_| Vec::new());
                        child
                    })
                    .collect::<Vec<File>>(),
            ),
        };
        return Ok(Some(file));
    }

    Ok(None)
}

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DriveInfo {
    pub root: String,
    pub letter: char,
}

impl From<PathBuf> for DriveInfo {
    fn from(path: PathBuf) -> Self {
        // Extract the drive letter of the target path
        let letter = path
            .components()
            .next()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .chars()
            .next()
            .unwrap();

        // Get the root of the target path
        let root = path
            .ancestors()
            .last()
            .unwrap()
            .to_string_lossy()
            .to_string(); // this unwrap is safe as .ancestors() always returns at least one value

        DriveInfo { letter, root }
    }
}

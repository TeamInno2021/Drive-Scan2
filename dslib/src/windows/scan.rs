use std::path::PathBuf;

pub fn scan(dir: PathBuf) {
    // Extract the drive letter of the target device
    let drive = dir
        .components()
        .next()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .chars()
        .next()
        .unwrap();

    println!("{:#?}", drive);
}

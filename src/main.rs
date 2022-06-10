use std::{
    fs::{metadata, read_dir},
    io,
    path::{Path, PathBuf},
};

use filetime::FileTime;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let working_directory = args.get(1).expect("working-directory is not specified");

    let path = PathBuf::from(working_directory);
    assert!(path.is_dir(), "{working_directory} is not a directory");
    if let Err(e) = reset_directory_mtime(path) {
        panic!("Failed to reset mtime: {e}");
    }
}

fn reset_directory_mtime<P: AsRef<Path>>(directory: P) -> io::Result<FileTime> {
    let mut recent_modified_time: Option<FileTime> = None;
    for entry in read_dir(&directory)? {
        let entry = entry?;
        let path = entry.path();
        let modified = if path.is_dir() {
            reset_directory_mtime(path)?
        } else {
            let metadata = metadata(path)?;
            FileTime::from_last_modification_time(&metadata)
        };
        recent_modified_time = match recent_modified_time {
            Some(current) => Some(current.max(modified)),
            None => Some(modified),
        };
    }

    let directory_modified_time = recent_modified_time.unwrap_or_else(FileTime::zero);
    filetime::set_file_mtime(&directory, directory_modified_time)?;
    Ok(directory_modified_time)
}

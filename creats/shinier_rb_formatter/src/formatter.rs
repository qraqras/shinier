use crate::printer::Printer;
use shinier_fs::FsRoot;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub struct Formatter {
    pub path: PathBuf,
    pub option: (),
}

impl Formatter {
    pub fn new(path: PathBuf, option: ()) -> Self {
        Self { path, option }
    }
    pub fn format(&self) {
        let fs_root = FsRoot::new(self.path.clone());
        for file_path in fs_root
            .descendant_files()
            .filter_map(|r| r.ok())
            .filter(|p| p.extension().and_then(OsStr::to_str) == Some("rb"))
        {
            match fs::read_to_string(&file_path) {
                Ok(contents) => {
                    Printer::new(contents, ()).print();
                }
                Err(e) => eprintln!("Error reading {:?}: {}", file_path, e),
            }
        }
    }
}

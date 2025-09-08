use ruby_prism::parse;
use shinier_fs::FsRoot;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub fn execute_formatting(root_paths: Vec<PathBuf>) {
    for root_path in root_paths.into_iter() {
        let fs_root = FsRoot::new(root_path);
        for file_path in fs_root
            .descendant_files()
            .filter_map(|r| r.ok())
            .filter(|p| p.extension().and_then(OsStr::to_str) == Some("rb"))
        {
            match fs::read_to_string(&file_path) {
                Ok(contents) => {
                    format_code(&file_path, &contents);
                }
                Err(e) => eprintln!("Error reading {:?}: {}", file_path, e),
            }
        }
    }
}

fn format_code(path: &PathBuf, src: &str) {
    println!("Formatting file: {:?}", path);
    println!("{:?}", parse(src.as_bytes()).node());
}

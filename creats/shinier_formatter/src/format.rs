use shinier_fs::FsRoot;
use std::path::PathBuf;

pub fn execute_formatting(root_paths: Vec<PathBuf>) {
    for fs_root in root_paths.iter().map(|p| FsRoot::new(p.clone())) {
        for file_result in fs_root.descendant_files() {
            match file_result {
                Ok(path) => {
                    println!("Formatting file: {:?}", path);
                    // Here you would call your actual formatting logic
                    format_code();
                }
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
    }
}

fn format_code() {}

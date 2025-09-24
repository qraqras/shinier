use std::collections::{HashSet, VecDeque};
use std::fs::{canonicalize, metadata, read_dir};
use std::io::Result;
use std::path::PathBuf;

pub struct FsRoot {
    pub path: PathBuf,
}
impl FsRoot {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn descendant_files(&self) -> impl Iterator<Item = Result<PathBuf>> {
        FileWalker::new(self.path.clone())
    }
}

struct FileWalker {
    pending: VecDeque<Result<PathBuf>>,
    visited_dirs: HashSet<PathBuf>,
}
impl FileWalker {
    fn new(root: PathBuf) -> Self {
        Self {
            pending: VecDeque::from([Ok(root)]),
            visited_dirs: HashSet::new(),
        }
    }
}
impl Iterator for FileWalker {
    type Item = Result<PathBuf>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(p) = self.pending.pop_back() {
            match p {
                Ok(p) => match metadata(&p) {
                    Ok(meta) if meta.is_file() => {
                        return Some(Ok(p));
                    }
                    Ok(meta) if meta.is_dir() => {
                        if let Ok(canon) = canonicalize(&p) {
                            if !self.visited_dirs.insert(canon) {
                                continue;
                            }
                        }
                        match read_dir(&p) {
                            Ok(entries) => {
                                for entry in entries {
                                    let item = entry.map(|en| en.path());
                                    self.pending.push_front(item);
                                }
                            }
                            Err(e) => {
                                return Some(Err(e));
                            }
                        }
                    }
                    Ok(_) => continue,
                    Err(e) => return Some(Err(e)),
                },
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

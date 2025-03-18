use std::env;
use std::fs;
use std::path::PathBuf;

struct FileExplorer {
    current_dir: PathBuf,
    entries: Vec<PathBuf>,
    selected_index: usize,
}

impl FileExplorer {
    fn new(start_dir: PathBuf) -> Self {
        let entries = Self::read_dir(&start_dir);
        Self {
            current_dir: (start_dir),
            entries,
            selected_index: (0),
        }
    }

    fn read_dir(path: &PathBuf) -> Vec<PathBuf> {
        fs::read_dir(path)
            .map(|entries| entries.filter_map(Result::ok).map(|e| e.path()).collect())
            .unwrap_or_default()
    }
}

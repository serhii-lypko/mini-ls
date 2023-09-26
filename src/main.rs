use std::fmt;
use std::fs;
use std::fs::{DirEntry, Metadata};
use std::io::Result;
use std::path::Path;

pub fn main() {
    Ls::execute();
}

pub struct Ls;

impl Ls {
    pub fn execute() -> Result<()> {
        let path_string = Ls::get_args_path_or_default();
        let root_path = Path::new(path_string.as_str());

        Ls::print_dir(&root_path)
    }

    fn get_args_path_or_default() -> String {
        use std::env;

        let args: Vec<String> = env::args().collect();
        let args_path = args.get(1).map(|path| path.to_owned());

        match args_path {
            Some(arg_path) => arg_path,
            None => ".".to_string(),
        }
    }

    fn print_dir(root_path: &Path) -> Result<()> {
        use fs::read_dir;

        let mut sorted_entries: Vec<String> = vec![];

        read_dir(root_path)?
            .map(EntryInfo::gather_data)
            .filter_map(Result::ok)
            .for_each(|entry_info| {
                if entry_info.meta.is_dir() {
                    sorted_entries.insert(0, entry_info.info_string);
                } else if entry_info.meta.is_file() {
                    sorted_entries.push(entry_info.info_string);
                }
            });

        let result_string = sorted_entries
            .into_iter()
            .fold("".to_string(), |cur, next| format!("{}{}\n", cur, next));

        println!("{}", result_string);

        Ok(())
    }
}

enum EntryType {
    Dir,
    File,
    Other,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntryType::Dir => write!(f, "dir"),
            EntryType::File => write!(f, "file"),
            EntryType::Other => write!(f, "other"),
        }
    }
}

impl From<Metadata> for EntryType {
    fn from(meta: Metadata) -> Self {
        if meta.is_dir() {
            Self::Dir
        } else if meta.is_file() {
            Self::File
        } else {
            Self::Other
        }
    }
}

struct EntryInfo {
    meta: Metadata,
    info_string: String,
}

impl EntryInfo {
    fn gather_data(entry: Result<DirEntry>) -> Result<Self> {
        use fs::metadata;

        let path = entry?.path();
        let metadata = metadata(&path)?;

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let base_type: EntryType = metadata.clone().into();

        let info_string = format!("{} {} {}", name, base_type, metadata.len());

        Ok(EntryInfo {
            meta: metadata,
            info_string,
        })
    }
}

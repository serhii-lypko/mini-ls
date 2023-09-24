use std::fmt;
use std::fs::{DirEntry, Metadata};
use std::io::Result;
use std::path::Path;
use std::{fs, vec};

pub struct Ls;

// TODO: formatting string so each column gows the same width in table

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

impl Ls {
    pub fn execute() -> Result<()> {
        let arg_path_string = Ls::get_path_string_from_args();
        let arg_path_string = match arg_path_string {
            Some(path_string) => path_string,
            None => ".".to_string(),
        };

        let root_path = Path::new(arg_path_string.as_str());

        Ls::print_dir(&root_path)
    }

    fn get_path_string_from_args() -> Option<String> {
        use std::env;

        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            let expected_arg_path = &args[1];
            return Some(expected_arg_path.to_owned());
        }

        None
    }

    fn print_dir(root_path: &Path) -> Result<()> {
        use fs::read_dir;

        let mut dirs_info: Vec<String> = vec![];
        let mut files_info: Vec<String> = vec![];

        for entry in read_dir(root_path)? {
            if let Ok(entry) = entry {
                if let Ok((metadata, entry_info)) = Ls::gather_entry_info(entry) {
                    if metadata.is_dir() {
                        dirs_info.push(entry_info);
                    } else {
                        files_info.push(entry_info);
                    }
                }
            }
        }

        dirs_info.append(&mut files_info);

        for dir_info in dirs_info {
            println!("{}", dir_info);
        }

        Ok(())
    }

    // TODO: better to use Struct with named fields for Result T?
    fn gather_entry_info(entry: DirEntry) -> Result<(Metadata, String)> {
        use fs::metadata;

        let path = entry.path();
        let metadata = metadata(&path)?;

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let base_type: EntryType = metadata.clone().into();

        let entry_info = format!("{} {} {}", name, base_type, metadata.len());

        Ok((metadata, entry_info))
    }
}

fn main() -> Result<()> {
    Ls::execute()
}

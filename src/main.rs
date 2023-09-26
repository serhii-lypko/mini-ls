use std::fmt;
use std::fs;
use std::fs::{DirEntry, Metadata};
use std::io::Result;
use std::path::Path;

// TODO: ordering of the structs & methods declaration?

pub fn main() {
    //
}

pub struct Ls;

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

    // TODO: rename -> path_or_default?
    fn get_path_string_from_args() -> Option<String> {
        use std::env;

        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            let expected_arg_path = &args[1];
            return Some(expected_arg_path.to_owned());
        }

        None
    }

    /*
        fs::read_dir(path) returns Result<ReadDir, Error>
        if success -> Iterator<Result<DirEntry, Error>>
    */

    // TODO: but how to implement sorting while mapping stuff
    // TODO: simple strategy is to simply put dir on top of the new list and files on the end
    fn print_dir(root_path: &Path) -> Result<()> {
        use fs::read_dir;

        // Iterator<Result<EntryInfo>>
        let entries_info_iter = read_dir(root_path)?.map(EntryInfo::gather_data);

        let res = entries_info_iter
            .filter_map(Result::ok)
            .fold("".to_string(), |cur, next| {
                format!("{}{}\n", cur, next.info_string.as_str())
            });

        println!("{}", res);

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

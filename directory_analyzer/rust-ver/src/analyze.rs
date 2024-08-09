use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use crate::{Args, FileExtension};
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct AnalyzedInfo {
    ///Count of directories found during the analysis
    found_dirs: u32,
    ///Count of files found during the analysis
    found_files: u32,
    ///Info about symlinks found during the analysis
    found_symlinks: Option<SymlinkInfo>,
    ///Info about files grouped by file type
    file_info: Option<HashMap<FileExtension, FileTypeInfo>>,
}

impl std::fmt::Display for AnalyzedInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symlinks_str = match self.found_symlinks {
            Some(sym) => format!("\n{sym}"),
            None => "".to_string(),
        };
        let info_str = match &self.file_info {
            Some(info) => {
                let mut out = String::new();
                for (file_ext, ft_info) in info {
                    out.push_str(&format!("\nFile extension:{file_ext}{ft_info}"));
                }
                out
            }
            None => "".to_string(),
        };
        let str = format!(
            "Found directories: {}\nFound files: {}{symlinks_str}{info_str}",
            self.found_dirs, self.found_files
        );
        write!(f, "{str}")
    }
}

impl AnalyzedInfo {
    ///Count of directories found during the analysis
    pub fn found_dirs(&self) -> u32 {
        self.found_dirs
    }
    ///Count of files found during the analysis
    pub fn found_files(&self) -> u32 {
        self.found_files
    }
    ///Info about files grouped by file type
    pub fn file_info(&self) -> Option<&HashMap<FileExtension, FileTypeInfo>> {
        self.file_info.as_ref()
    }

    pub fn found_symlinks(&self) -> Option<&SymlinkInfo> {
        self.found_symlinks.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileTypeInfo {
    num_files: u32,
    largest_file: FileTypeInfoRecords,
    smallest_file: FileTypeInfoRecords,
    ///Total size of all files of this type
    size_in_bytes: u64,
}

impl std::fmt::Display for FileTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n  Number of files:{}\n  Largest file: {}\n  Smallest file: {}\n  Size in bytes for this type: {}",
            self.num_files, self.largest_file, self.smallest_file, self.size_in_bytes
        )
    }
}

///This is for holding info about specific notable files
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct FileTypeInfoRecords {
    ///Path to the path
    path: PathBuf,
    ///The size of the file
    size: u64,
}

impl std::fmt::Display for FileTypeInfoRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n    Path:{}\n    Size: {}",
            self.path.to_string_lossy(),
            self.size
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy, Serialize)]
pub struct SymlinkInfo {
    found_symlinks: u32,
    ///Number of symlinks that point to files
    file_symlinks: u32,
    ///Number of symlinks that point to directories
    dir_symlinks: u32,
}

impl std::fmt::Display for SymlinkInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"\nFound symbolic links:{}\nSymlinks that point to files: {}\nSymlinks that point to directories: {}",self.found_symlinks,self.file_symlinks,self.dir_symlinks)
    }
}

impl SymlinkInfo {
    pub fn new(found_symlinks: u32, file_symlinks: u32, dir_symlinks: u32) -> Self {
        Self {
            found_symlinks,
            file_symlinks,
            dir_symlinks,
        }
    }

    ///Number of symlinks that point to files
    pub fn file_symlinks(&self) -> u32 {
        self.file_symlinks
    }

    pub fn found_symlinks(&self) -> u32 {
        self.found_symlinks
    }

    ///Number of symlinks that point to directories
    pub fn dir_symlinks(&self) -> u32 {
        self.dir_symlinks
    }
}

impl FileTypeInfoRecords {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self { path, size }
    }

    ///Path to the path
    pub fn path(&self) -> &Path {
        &self.path.as_path()
    }
    ///The size of the file
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl FileTypeInfo {
    pub fn new() -> Self {
        Self {
            num_files: u32::default(),
            size_in_bytes: u64::default(),
            largest_file: FileTypeInfoRecords::default(),
            smallest_file: FileTypeInfoRecords::default(),
        }
    }

    pub fn size_in_bytes(&self) -> u64 {
        self.size_in_bytes
    }

    pub fn num_files(&self) -> u32 {
        self.num_files
    }
    
    pub fn largest_file(&self) -> &FileTypeInfoRecords {
        &self.largest_file
    }
    
    pub fn smallest_file(&self) -> &FileTypeInfoRecords {
        &self.smallest_file
    }
}

pub fn analyze(args: &Args) -> Result<AnalyzedInfo> {
    let mut out = set_up_anaylzed_info(args);
    search_dir(args, &mut out)?;
    if args.follow_symlinks() {
        debug_assert_eq!(
            out.found_symlinks.unwrap().found_symlinks,
            out.found_symlinks.unwrap().dir_symlinks + out.found_symlinks.unwrap().file_symlinks
        );
    }
    Ok(out)
}

fn search_dir(args: &Args, analyed_info: &mut AnalyzedInfo) -> Result<()> {
    let mut dirs_to_analyze = vec![args.path_to_analyze().clone()];
    //used to prevent repeatedly counting the same item multiple times while following symlinks
    let mut found_items = Vec::new();
    while let Some(dir) = dirs_to_analyze.pop() {
        let cur_dir = dir.read_dir()?;
        for entry in cur_dir {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let cannonical_path = entry.path().canonicalize()?;
            if metadata.is_dir() && !found_items.contains(&cannonical_path) {
                handle_dirs(args, &mut dirs_to_analyze, &entry, analyed_info);
                if args.follow_symlinks() {
                    found_items.push(cannonical_path);
                }
            } else if metadata.is_file() && !found_items.contains(&cannonical_path) {
                handle_files(args, analyed_info, &entry, &metadata);
                if args.follow_symlinks() {
                    found_items.push(cannonical_path);
                }
            } else if args.follow_symlinks()
                && metadata.is_symlink()
                && !found_items.contains(&cannonical_path)
            {
                let path = fs::read_link(entry.path())?;
                let metadata = path.metadata()?;
                let cannonical_path = path.canonicalize()?;

                if !found_items.contains(&cannonical_path) {
                    if metadata.is_dir() {
                        handle_dirs(args, &mut dirs_to_analyze, &entry, analyed_info);
                        if let Some(symlink) = &mut analyed_info.found_symlinks {
                            symlink.dir_symlinks += 1;
                            symlink.found_symlinks += 1;
                        }
                    } else if metadata.is_file() {
                        handle_files(args, analyed_info, &entry, &metadata);
                        if let Some(symlink) = &mut analyed_info.found_symlinks {
                            symlink.file_symlinks += 1;
                            symlink.found_symlinks += 1;
                        }
                    }
                    found_items.push(cannonical_path);
                }
            }
        }
    }

    fn handle_dirs(
        args: &Args,
        dirs_to_analyze: &mut Vec<PathBuf>,
        entry: &DirEntry,
        analyed_info: &mut AnalyzedInfo,
    ) {
        if !args.no_recurse() {
            dirs_to_analyze.push(entry.path());
        }
        if args.verbose() {
            println!(
                "dir: {}",
                entry.path().canonicalize().unwrap().to_string_lossy()
            );
        }
        analyed_info.found_dirs += 1;
    }

    fn handle_files(
        args: &Args,
        analyed_info: &mut AnalyzedInfo,
        entry: &DirEntry,
        metadata: &fs::Metadata,
    ) {
        if args.verbose() {
            println!(
                "file: {}",
                entry.path().canonicalize().unwrap().to_string_lossy()
            );
        }
        analyed_info.found_files += 1;
        if let Some(map) = &mut analyed_info.file_info {
            match entry.path().extension() {
                Some(ext) => {
                    let ext = ext.to_os_string().to_string_lossy().to_string();
                    add_file_info_to_map(ext, map, entry, metadata);
                }
                //Still want to keep info about files without extensions
                None => {
                    let ext = "".to_string();
                    add_file_info_to_map(ext, map, entry, metadata);
                }
            }
        }
    }
    Ok(())
}

fn add_file_info_to_map(
    extension: String,
    map: &mut HashMap<String, FileTypeInfo>,
    entry: &DirEntry,
    metadata: &std::fs::Metadata,
) {
    let t = map.entry(extension).or_insert(FileTypeInfo {
        num_files: 0,
        size_in_bytes: 0,
        largest_file: FileTypeInfoRecords::default(),
        smallest_file: FileTypeInfoRecords::new(PathBuf::default(), u64::MAX),
    });
    t.num_files += 1;
    t.size_in_bytes += metadata.len();
    if metadata.len() > t.largest_file.size() {
        t.largest_file = FileTypeInfoRecords::new(entry.path(), metadata.len());
    }
    if metadata.len() < t.smallest_file.size() {
        t.smallest_file = FileTypeInfoRecords::new(entry.path(), metadata.len());
    }
}

///Configures `AnalyzedInfo` based on the `Args` given
fn set_up_anaylzed_info(args: &Args) -> AnalyzedInfo {
    let mut out = AnalyzedInfo::default();
    if args.file_info() {
        out.file_info = Some(HashMap::default());
    }
    if args.follow_symlinks() {
        out.found_symlinks = Some(SymlinkInfo::default());
    }
    out
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::*;

    const TEST_DIR: &str = "../test/";

    #[test]
    fn analyze_default_settings() {
        let test_args = Args::new(
            PathBuf::from_str(TEST_DIR).unwrap(),
            false,
            false,
            false,
            false,
            None,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo {
            found_dirs: 4,
            found_files: 7,
            found_symlinks: None,
            file_info: None,
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn analyze_no_recurse() {
        let test_args = Args::new(
            PathBuf::from_str(TEST_DIR).unwrap(),
            true,
            false,
            false,
            false,
            None,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo {
            found_dirs: 2,
            found_files: 4,
            found_symlinks: None,
            file_info: None,
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn analyze_file_info() {
        let test_args = Args::new(
            PathBuf::from_str(TEST_DIR).unwrap(),
            false,
            true,
            false,
            false,
            None,
        );
        let res = analyze(&test_args).unwrap();
        let mut hash_map: HashMap<FileExtension, FileTypeInfo> = HashMap::new();
        //Byte values are from windows properties Size: field for each file

        hash_map.insert(
            "txt".to_string(),
            FileTypeInfo {
                size_in_bytes: 9,
                num_files: 1,
                largest_file: FileTypeInfoRecords::default(),
                smallest_file: FileTypeInfoRecords::default(),
            },
        ); //file1.txt
        let h = hash_map.entry("txt".to_string()).or_insert(FileTypeInfo {
            size_in_bytes: 14,
            num_files: 1,
            largest_file: FileTypeInfoRecords::default(),
            smallest_file: FileTypeInfoRecords::default(),
        }); //file2.txt
        h.size_in_bytes += 14;
        h.num_files += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file2.txt");
        h.largest_file = FileTypeInfoRecords::new(path, 14);
        let h = hash_map.entry("txt".to_string()).or_insert(FileTypeInfo {
            size_in_bytes: 9,
            num_files: 1,
            largest_file: FileTypeInfoRecords::default(),
            smallest_file: FileTypeInfoRecords::default(),
        }); //folder1/file5.txt
        h.size_in_bytes += 9;
        h.num_files += 1;
        let h = hash_map.entry("txt".to_string()).or_insert(FileTypeInfo {
            size_in_bytes: 7,
            num_files: 1,
            largest_file: FileTypeInfoRecords::default(),
            smallest_file: FileTypeInfoRecords::default(),
        }); //folder2/file6.txt
        h.size_in_bytes += 7;
        h.num_files += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("folder2/file6.txt");
        h.smallest_file = FileTypeInfoRecords::new(path, 7);
        let h = hash_map.entry("txt".to_string()).or_insert(FileTypeInfo {
            size_in_bytes: 9,
            num_files: 1,
            largest_file: FileTypeInfoRecords::default(),
            smallest_file: FileTypeInfoRecords::default(),
        }); //folder2/folder3/folder4/deepfile1.txt
        h.size_in_bytes += 9;
        h.num_files += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file3.rtf");
        hash_map.insert(
            "rtf".to_string(),
            FileTypeInfo {
                size_in_bytes: 196,
                num_files: 1,
                largest_file: FileTypeInfoRecords::new(path.clone(), 196),
                smallest_file: FileTypeInfoRecords::new(path, 196),
            },
        ); //file3.rtf
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file4.zip");
        hash_map.insert(
            "zip".to_string(),
            FileTypeInfo {
                size_in_bytes: 188,
                num_files: 1,
                largest_file: FileTypeInfoRecords::new(path.clone(), 188),
                smallest_file: FileTypeInfoRecords::new(path.clone(), 188),
            },
        ); //file4.zip
        let expected = AnalyzedInfo {
            found_dirs: 4,
            found_files: 7,
            found_symlinks: None,
            file_info: Some(hash_map),
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn analyze_follow_symlinks() {
        let test_args = Args::new(
            PathBuf::from_str(TEST_DIR).unwrap(),
            false,
            false,
            true,
            false,
            None,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo {
            found_dirs: 6,
            found_files: 8,
            found_symlinks: Some(SymlinkInfo::new(2, 1, 1)),
            file_info: None,
        };
        assert_eq!(res, expected);
    }
}

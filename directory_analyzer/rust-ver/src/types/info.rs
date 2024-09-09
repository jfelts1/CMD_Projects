use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Serialize;

use super::FileExtension;

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
    ///How many bytes taken up by all the files
    total_bytes: u64,
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
            "Found directories: {}\nFound files: {}\nTotal bytes: {} bytes{symlinks_str}\n{info_str}",
            self.found_dirs, self.found_files, self.total_bytes
        );
        write!(f, "{str}")
    }
}

impl AnalyzedInfo {
    pub fn new(
        found_dirs: u32,
        found_files: u32,
        found_symlinks: Option<SymlinkInfo>,
        file_info: Option<HashMap<FileExtension, FileTypeInfo>>,
        total_bytes: u64,
    ) -> Self {
        Self {
            found_dirs,
            found_files,
            found_symlinks,
            file_info,
            total_bytes,
        }
    }

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

    pub fn found_dirs_mut(&mut self) -> &mut u32 {
        &mut self.found_dirs
    }

    pub fn found_files_mut(&mut self) -> &mut u32 {
        &mut self.found_files
    }

    pub fn set_found_symlinks(&mut self, found_symlinks: Option<SymlinkInfo>) {
        self.found_symlinks = found_symlinks;
    }

    pub fn set_file_info(&mut self, file_info: Option<HashMap<FileExtension, FileTypeInfo>>) {
        self.file_info = file_info;
    }

    ///Returns a mutable reference to found_symlinks if it is Some()
    pub fn found_symlinks_mut(&mut self) -> Option<&mut SymlinkInfo> {
        if let Some(symlink) = &mut self.found_symlinks {
            return Some(symlink);
        }
        None
    }

    ///Returns a mutable reference to file_info if it is Some()
    pub fn file_info_mut(&mut self) -> Option<&mut HashMap<FileExtension, FileTypeInfo>> {
        if let Some(file_info) = &mut self.file_info {
            return Some(file_info);
        }
        None
    }

    ///Calculates the the percent of total files and percent of total size on disk for each type of file,if file info is gathered.
    pub fn calculate_percentages_for_info(&mut self) {
        if let Some(map) = &mut self.file_info {
            for info in map.values_mut() {
                info.calculate_percentages(self.total_bytes, self.found_files);
            }
        }
    }

    ///How many bytes taken up by all the files
    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn total_bytes_mut(&mut self) -> &mut u64 {
        &mut self.total_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct FileTypeInfo {
    num_files: u32,
    largest_file: FileTypeInfoRecords,
    smallest_file: FileTypeInfoRecords,
    ///Total size of all files of this type
    size_in_bytes: u64,
    percent_of_total_files: Option<f32>,
    percent_of_total_size: Option<f32>,
}

impl std::fmt::Display for FileTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (per_tot_files, per_tot_size) = self.percentages_in_string();
        let per_tot_files = format!(" % of total files: {per_tot_files}\n");
        let per_tot_size = format!("\n  % of total size: {per_tot_size}");
        write!(
            f,
            "\n  Number of files:{}\n{}  Largest file: {}\n  Smallest file: {}\n  Size in bytes for this type: {}{}",
            self.num_files,per_tot_files, self.largest_file, self.smallest_file, self.size_in_bytes,per_tot_size
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

    pub fn found_symlinks_mut(&mut self) -> &mut u32 {
        &mut self.found_symlinks
    }

    pub fn file_symlinks_mut(&mut self) -> &mut u32 {
        &mut self.file_symlinks
    }

    pub fn dir_symlinks_mut(&mut self) -> &mut u32 {
        &mut self.dir_symlinks
    }
}

impl FileTypeInfoRecords {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self { path, size }
    }

    ///Path to the path
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    ///The size of the file
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl FileTypeInfo {
    pub fn new(
        size_in_bytes: u64,
        num_files: u32,
        largest_file: FileTypeInfoRecords,
        smallest_file: FileTypeInfoRecords,
    ) -> Self {
        Self {
            num_files,
            largest_file,
            smallest_file,
            size_in_bytes,
            percent_of_total_files: None,
            percent_of_total_size: None,
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

    pub fn num_files_mut(&mut self) -> &mut u32 {
        &mut self.num_files
    }

    pub fn size_in_bytes_mut(&mut self) -> &mut u64 {
        &mut self.size_in_bytes
    }

    pub fn set_largest_file(&mut self, largest_file: FileTypeInfoRecords) {
        self.largest_file = largest_file;
    }

    pub fn set_smallest_file(&mut self, smallest_file: FileTypeInfoRecords) {
        self.smallest_file = smallest_file;
    }

    ///Calculates `percent_of_total_files` and `percent_of_total_size`
    pub fn calculate_percentages(&mut self, total_bytes: u64, total_files: u32) {
        self.percent_of_total_files = Some(self.num_files as f32 / total_files as f32);
        self.percent_of_total_size = Some(self.size_in_bytes as f32 / total_bytes as f32);
    }

    pub fn percentages_in_string(&self) -> (PercentageOfFiles, PercentageOfSize) {
        let per_tot_files = match self.percent_of_total_files {
            Some(per) => {
                let tmp = format!("{:.2}", per * 100.0);
                if tmp == "0.00" {
                    "< 0.01".to_string()
                } else {
                    tmp
                }
            }
            None => "N/A\n".to_string(),
        };
        let per_tot_size = match self.percent_of_total_size {
            Some(per) => {
                let tmp = format!("{:.2}", per * 100.0);
                if tmp == "0.00" {
                    "< 0.01".to_string()
                } else {
                    tmp
                }
            }
            None => "N/A".to_string(),
        };
        (per_tot_files, per_tot_size)
    }
}

pub type PercentageOfFiles = String;
pub type PercentageOfSize = String;

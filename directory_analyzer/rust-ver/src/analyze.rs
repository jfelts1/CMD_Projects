use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::{self, PathBuf},
};

use crate::{AnalyzedInfo, Args, FastPath, FileTypeInfo, FileTypeInfoRecords, SymlinkInfo, Timer};
use anyhow::Result;

pub fn analyze(args: &Args) -> Result<AnalyzedInfo> {
    let mut out = set_up_anaylzed_info(args);
    search_dirs(args, &mut out)?;
    out.calculate_percentages_for_info();
    if let Some(sym) = out.found_symlinks() {
        //Sanity check to make sure things add up
        debug_assert_eq!(
            sym.found_symlinks(),
            sym.dir_symlinks() + sym.file_symlinks()
        );
    }
    Ok(out)
}

fn search_dirs(args: &Args, analyed_info: &mut AnalyzedInfo) -> Result<()> {
    let mut dirs_to_analyze = vec![args.path_to_analyze().clone()];
    //used to prevent repeatedly counting the same item multiple times while following symlinks
    let mut found_items = Vec::new();
    let mut timer = args.updates().map(Timer::new);
    let ignore_these = set_up_ignore_these(args)?;
    while let Some(dir) = dirs_to_analyze.pop() {
        let cur_dir = dir.read_dir()?;
        for entry in cur_dir {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let path = path::absolute(entry.path())?;
            if let Some(ignore_these) = &ignore_these {
                if ignore_these
                    .iter()
                    .any(|p| path::absolute(p).unwrap() == path)
                {
                    continue;
                }
            }
            if metadata.is_dir() && !found_items.contains(&FastPath::new(&path)) {
                handle_dirs(args, &mut dirs_to_analyze, &entry, analyed_info)?;
                //No need to track found items since there is no risk of loops or double counting
                if args.follow_symlinks() {
                    found_items.push(FastPath::new(&path));
                }
            } else if metadata.is_file() && !found_items.contains(&FastPath::new(&path)) {
                handle_files(args, analyed_info, &entry, &metadata)?;
                //No need to track found items since there is no risk of loops or double counting
                if args.follow_symlinks() {
                    found_items.push(FastPath::new(&path));
                }
            } else if (args.count_symlinks() || args.follow_symlinks())
                && metadata.is_symlink()
                && !found_items.contains(&FastPath::new(&path))
            {
                handle_symlinks(
                    entry,
                    &mut found_items,
                    args,
                    &mut dirs_to_analyze,
                    analyed_info,
                )?;
            }
        }
        if let Some(timer) = &mut timer {
            timer.update();
            if timer.ended() {
                println!(
                    "Update: {} found dirs, {} found files",
                    analyed_info.found_dirs(),
                    analyed_info.found_files()
                );
                timer.reset();
            }
        }
    }

    fn handle_dirs(
        args: &Args,
        dirs_to_analyze: &mut Vec<PathBuf>,
        entry: &DirEntry,
        analyed_info: &mut AnalyzedInfo,
    ) -> anyhow::Result<()> {
        handle_args(args, dirs_to_analyze, entry)?;
        *analyed_info.found_dirs_mut() += 1;

        fn handle_args(
            args: &Args,
            dirs_to_analyze: &mut Vec<PathBuf>,
            entry: &DirEntry,
        ) -> anyhow::Result<()> {
            if !args.no_recurse() {
                dirs_to_analyze.push(entry.path());
            }
            if args.verbose() {
                if args.full_path() {
                    println!("dir: {}", path::absolute(entry.path())?.to_string_lossy());
                } else {
                    println!("dir: {}", entry.path().to_string_lossy());
                }
            }
            Ok(())
        }
        Ok(())
    }

    fn handle_files(
        args: &Args,
        analyed_info: &mut AnalyzedInfo,
        entry: &DirEntry,
        metadata: &fs::Metadata,
    ) -> anyhow::Result<()> {
        handle_file_args(args, analyed_info.file_info_mut(), entry, metadata)?;
        *analyed_info.found_files_mut() += 1;
        *analyed_info.total_bytes_mut() += metadata.len();

        fn handle_file_args(
            args: &Args,
            map: Option<&mut HashMap<String, FileTypeInfo>>,
            entry: &DirEntry,
            metadata: &fs::Metadata,
        ) -> anyhow::Result<()> {
            if args.verbose() {
                if args.full_path() {
                    println!("file: {}", path::absolute(entry.path())?.to_string_lossy())
                } else {
                    println!("file: {}", entry.path().to_string_lossy())
                }
            }

            if let Some(map) = map {
                match entry.path().extension() {
                    Some(ext) => {
                        let ext = ext.to_os_string().to_string_lossy().to_string();
                        add_file_info_to_map(args, ext, map, entry, metadata)?;
                    }
                    //Still want to keep info about files without extensions
                    None => {
                        let ext = "".to_string();
                        add_file_info_to_map(args, ext, map, entry, metadata)?;
                    }
                }
            }
            Ok(())
        }
        Ok(())
    }

    ///Traverses and counts symlinks and if the target is not already counted counts it
    fn handle_symlinks(
        entry: DirEntry,
        found_items: &mut Vec<FastPath>,
        args: &Args,
        dirs_to_analyze: &mut Vec<PathBuf>,
        analyed_info: &mut AnalyzedInfo,
    ) -> Result<(), anyhow::Error> {
        let path = fs::read_link(entry.path())?;
        let metadata = path.metadata()?;
        if args.follow_symlinks() {
            //don't look at entries that have been seen before
            //prevents following symlink loops and counting entries multiple times
            if !found_items.contains(&FastPath::new(&path)) {
                if metadata.is_dir() {
                    handle_dirs(args, dirs_to_analyze, &entry, analyed_info)?;
                } else if metadata.is_file() {
                    handle_files(args, analyed_info, &entry, &metadata)?;
                }

                found_items.push(FastPath::new(&path));
            }
        }

        if args.count_symlinks() {
            //Count the found symlinks here since a symlink can be new but point to a entry that
            //has already been seen before. We still want to count the symlink as found though
            if let Some(symlink) = analyed_info.found_symlinks_mut() {
                if metadata.is_dir() {
                    *symlink.dir_symlinks_mut() += 1;
                } else if metadata.is_file() {
                    *symlink.file_symlinks_mut() += 1;
                }
                *symlink.found_symlinks_mut() += 1;
            }
        }

        Ok(())
    }

    Ok(())
}

fn set_up_ignore_these(args: &Args) -> Result<Option<Vec<PathBuf>>, anyhow::Error> {
    let ignore_these: Option<Vec<_>> = if let Some(s) = args.ignore_entries() {
        let paths: Vec<_> = s.split(',').map(|s| s.trim()).map(PathBuf::from).collect();
        for p in &paths {
            let exists = p.try_exists()?;
            if !exists {
                eprintln!(
                    "WARNING: Can't ignore \"{}\" because it doesn't exist",
                    p.to_string_lossy()
                );
            }
        }
        Some(paths)
    } else {
        None
    };
    Ok(ignore_these)
}

fn add_file_info_to_map(
    args: &Args,
    extension: String,
    map: &mut HashMap<String, FileTypeInfo>,
    entry: &DirEntry,
    metadata: &std::fs::Metadata,
) -> anyhow::Result<()> {
    let t = map.entry(extension).or_insert(FileTypeInfo::new(
        0,
        0,
        FileTypeInfoRecords::default(),
        FileTypeInfoRecords::new(PathBuf::default(), u64::MAX),
    ));
    *t.num_files_mut() += 1;
    *t.size_in_bytes_mut() += metadata.len();
    let path = if args.full_path() {
        path::absolute(entry.path())?
    } else {
        entry.path()
    };
    if metadata.len() > t.largest_file().size() {
        t.set_largest_file(FileTypeInfoRecords::new(path.clone(), metadata.len()));
    }
    if metadata.len() < t.smallest_file().size() {
        t.set_smallest_file(FileTypeInfoRecords::new(path, metadata.len()));
    }
    Ok(())
}

///Configures `AnalyzedInfo` based on the `Args` given
fn set_up_anaylzed_info(args: &Args) -> AnalyzedInfo {
    let mut out = AnalyzedInfo::default();
    if args.file_info() {
        out.set_file_info(Some(HashMap::default()));
    }
    if args.count_symlinks() {
        out.set_found_symlinks(Some(SymlinkInfo::default()));
    }
    out
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::FileExtension;

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
            false,
            None,
            None,
            true,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(4, 7, Some(SymlinkInfo::new(2, 1, 1)), None, 432);
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
            false,
            None,
            None,
            false,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(2, 4, None, None, 407);
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
            false,
            None,
            None,
            false,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let mut hash_map: HashMap<FileExtension, FileTypeInfo> = HashMap::new();
        //Byte values are from windows properties Size: field for each file

        hash_map.insert(
            "txt".to_string(),
            FileTypeInfo::new(
                9,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            ),
        ); //file1.txt
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                14,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //file2.txt
        *h.size_in_bytes_mut() += 14;
        *h.num_files_mut() += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file2.txt");
        h.set_largest_file(FileTypeInfoRecords::new(path, 14));
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                9,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //folder1/file5.txt
        *h.size_in_bytes_mut() += 9;
        *h.num_files_mut() += 1;
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                7,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //folder2/file6.txt
        *h.size_in_bytes_mut() += 7;
        *h.num_files_mut() += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("folder2/file6.txt");
        h.set_smallest_file(FileTypeInfoRecords::new(path, 7));
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                9,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //folder2/folder3/folder4/deepfile1.txt
        *h.size_in_bytes_mut() += 9;
        *h.num_files_mut() += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file3.rtf");
        hash_map.insert(
            "rtf".to_string(),
            FileTypeInfo::new(
                196,
                1,
                FileTypeInfoRecords::new(path.clone(), 196),
                FileTypeInfoRecords::new(path, 196),
            ),
        ); //file3.rtf
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file4.zip");
        hash_map.insert(
            "zip".to_string(),
            FileTypeInfo::new(
                188,
                1,
                FileTypeInfoRecords::new(path.clone(), 188),
                FileTypeInfoRecords::new(path.clone(), 188),
            ),
        ); //file4.zip

        let (total_files, total_bytes) = get_total_files_and_bytes_from_map(&hash_map);
        for (_, info) in hash_map.iter_mut() {
            info.calculate_percentages(total_bytes, total_files);
        }
        let expected = AnalyzedInfo::new(4, 7, None, Some(hash_map), 432);
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
            false,
            None,
            None,
            true,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(6, 8, Some(SymlinkInfo::new(2, 1, 1)), None, 432);
        assert_eq!(res, expected);
    }

    #[test]
    fn analyze_ignore_entries() {
        let path = PathBuf::from_str(TEST_DIR).unwrap();
        let mut str1 = TEST_DIR.to_string();
        str1.push_str("folder2/folder3");
        let mut str2 = TEST_DIR.to_string();
        str2.push_str("file4.zip");
        let ignore_entries = format!("{str1}, {str2}");
        let test_args = Args::new(
            path,
            false,
            false,
            false,
            false,
            None,
            false,
            Some(ignore_entries),
            None,
            false,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(2, 5, None, None, 235);
        assert_eq!(res, expected);
    }

    #[test]
    fn analyze_ignore_entries_file_info() {
        let path = PathBuf::from_str(TEST_DIR).unwrap();
        let mut str1 = TEST_DIR.to_string();
        str1.push_str("folder2/folder3");
        let mut str2 = TEST_DIR.to_string();
        str2.push_str("file4.zip");
        let ignore_entries = format!("{str1}, {str2}");
        let test_args = Args::new(
            path,
            false,
            true,
            false,
            false,
            None,
            false,
            Some(ignore_entries),
            None,
            false,
            false,
        );
        let mut hash_map: HashMap<FileExtension, FileTypeInfo> = HashMap::new();
        //Byte values are from windows properties Size: field for each file

        hash_map.insert(
            "txt".to_string(),
            FileTypeInfo::new(
                9,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            ),
        ); //file1.txt
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                14,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //file2.txt
        *h.size_in_bytes_mut() += 14;
        *h.num_files_mut() += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file2.txt");
        h.set_largest_file(FileTypeInfoRecords::new(path, 14));
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                9,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //folder1/file5.txt
        *h.size_in_bytes_mut() += 9;
        *h.num_files_mut() += 1;
        let h = hash_map
            .entry("txt".to_string())
            .or_insert(FileTypeInfo::new(
                7,
                1,
                FileTypeInfoRecords::default(),
                FileTypeInfoRecords::default(),
            )); //folder2/file6.txt
        *h.size_in_bytes_mut() += 7;
        *h.num_files_mut() += 1;
        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("folder2/file6.txt");
        h.set_smallest_file(FileTypeInfoRecords::new(path, 7));

        let mut path = PathBuf::from_str(TEST_DIR).unwrap();
        path.push("file3.rtf");
        hash_map.insert(
            "rtf".to_string(),
            FileTypeInfo::new(
                196,
                1,
                FileTypeInfoRecords::new(path.clone(), 196),
                FileTypeInfoRecords::new(path, 196),
            ),
        ); //file3.rtf

        let (total_files, total_bytes) = get_total_files_and_bytes_from_map(&hash_map);
        for (_, info) in hash_map.iter_mut() {
            info.calculate_percentages(total_bytes, total_files);
        }
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(2, 5, None, Some(hash_map), 235);
        assert_eq!(res, expected);
    }

    fn get_total_files_and_bytes_from_map(hash_map: &HashMap<String, FileTypeInfo>) -> (u32, u64) {
        let total_files = hash_map
            .iter()
            .fold(0, |acc, (_, info)| acc + info.num_files());
        let total_bytes = hash_map
            .iter()
            .fold(0, |acc, (_, info)| acc + info.size_in_bytes());
        (total_files, total_bytes)
    }

    #[test]
    fn analyze_symlink_loop() {
        let test_args = Args::new(
            PathBuf::from_str("../test_symlink_loop/").unwrap(),
            false,
            false,
            true,
            false,
            None,
            false,
            None,
            None,
            true,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(2, 2, Some(SymlinkInfo::new(2, 0, 2)), None, 21);
        assert_eq!(res, expected);
    }

    #[test]
    fn analyze_follow_symlinks_no_count_symlinks() {
        let test_args = Args::new(
            PathBuf::from_str(TEST_DIR).unwrap(),
            false,
            false,
            true,
            false,
            None,
            false,
            None,
            None,
            false,
            false,
        );
        let res = analyze(&test_args).unwrap();
        let expected = AnalyzedInfo::new(6, 8, None, None, 432);
        assert_eq!(res, expected);
    }
}

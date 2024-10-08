use std::path::PathBuf;

use clap::Parser;

const L_ABOUT:&str = "This program is used to analyze a directory and tell you about the contents.";

#[derive(Parser, Debug)]
#[command(version,about,long_about = L_ABOUT)]
pub struct Args {
    ///Requied path to analyze
    path_to_analyze: PathBuf,

    ///Just look at the files in the current directory
    #[arg(short, long, default_value_t = false)]
    no_recurse: bool,

    ///Output info about what file types were found, such as type and size
    #[arg(short = 'f', long, default_value_t = false)]
    file_info: bool,

    ///Follow symlinks as if they were a normal object.
    /// 
    /// PERF NOTE: For very large directories(>100,000 files or directories) this can be slow to process.
    #[arg(short = 's', long, default_value_t = false)]
    follow_symlinks: bool,

    ///Counts symlinks found but does not follow them
    #[arg(short = 'c', long, default_value_t = false)]
    count_symlinks: bool,

    ///Print each object as it is found
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    ///Export info to a xlsx file at the location specified
    #[arg(short, long)]
    export_xlsx: Option<PathBuf>,

    ///When displaying paths should they be printed in full
    #[arg(short = 'p', long, default_value_t = false)]
    full_path: bool,

    ///Comma seperated list of directories and files that will be not included in the analysis
    #[arg(short = 'i', long)]
    ignore_entries: Option<String>,

    ///Provides periodic updates about how many entries have been analyized. Value is seconds between updates.
    #[arg(short, long)]
    updates: Option<u64>,

    ///Displays the time the program took to run to stdout
    #[arg(short, long, default_value_t = false)]
    time: bool,
}

impl Args {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path_to_analyze: PathBuf,
        no_recurse: bool,
        file_info: bool,
        follow_symlinks: bool,
        verbose: bool,
        export_xlsx: Option<PathBuf>,
        full_path: bool,
        ignore_entries: Option<String>,
        updates: Option<u64>,
        count_symlinks: bool,
        time: bool,
    ) -> Self {
        Self {
            path_to_analyze,
            no_recurse,
            file_info,
            follow_symlinks,
            verbose,
            export_xlsx,
            full_path,
            ignore_entries,
            updates,
            count_symlinks,
            time,
        }
    }

    ///Requied path to analyze
    pub fn path_to_analyze(&self) -> &PathBuf {
        &self.path_to_analyze
    }

    ///Just look at the files in the current directory
    pub fn no_recurse(&self) -> bool {
        self.no_recurse
    }

    ///Output info about what file types were found, such as type and size
    pub fn file_info(&self) -> bool {
        self.file_info
    }

    ///Follow symlinks as if they were a normal object
    pub fn follow_symlinks(&self) -> bool {
        self.follow_symlinks
    }

    ///Print each object as it is found
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn export_xlsx(&self) -> Option<&PathBuf> {
        self.export_xlsx.as_ref()
    }

    ///When displaying paths should they be printed in full
    pub fn full_path(&self) -> bool {
        self.full_path
    }

    ///Comma seperated list of directories and files that will be not included in the analysis
    pub fn ignore_entries(&self) -> Option<&String> {
        self.ignore_entries.as_ref()
    }

    ///Provides periodic updates about how many entries have been analyized. Value in Option is seconds between updates.
    pub fn updates(&self) -> Option<u64> {
        self.updates
    }

    ///Counts symlinks found but does not follow them
    pub fn count_symlinks(&self) -> bool {
        self.count_symlinks
    }

    ///Displays the time the program took to run to stdout
    pub fn time(&self) -> bool {
        self.time
    }
}

using CommandLine;

namespace dira.types
{
    public readonly struct Args(string path_to_analyze, bool no_recurse, bool file_info, bool follow_symlinks, bool count_symlinks, bool verbose, string? export_xlsx, bool full_path, string? ignore_entries, ulong? updates, bool time)
    {
        [Option('r', "path_to_analyze", Required = true, HelpText = "Requied path to analyze")]
        public string PathToAnalyze { get; } = path_to_analyze;

        [Option('n', "no_recurse", Required = false, HelpText = "Just look at the files in the current directory", Default = false)]
        public bool NoRecurse { get; } = no_recurse;

        [Option('f', "file_info", Required = false, HelpText = "Output info about what file types were found, such as type and size", Default = false)]
        public bool FileInfo { get; } = file_info;

        [Option('s', "follow_symlinks", Required = false, HelpText = "Follow symlinks as if they were a normal object.\nNOTE: Due to the need to track found files and directories to prevent counting the same entry multiple times, this has a noticable performance impact.Due to this unless you need to actually follow symlinks it is recomended to not use this flag.", Default = false)]
        public bool FollowSymlinks { get; } = follow_symlinks;

        [Option('c', "count_symlinks", Required = false, HelpText = "Counts symlinks found but does not follow them", Default = false)]
        public bool CountSymlinks { get; } = count_symlinks;

        [Option('v', "verbose", Required = false, HelpText = "Print each object as it is found", Default = false)]
        public bool Verbose { get; } = verbose;

        [Option('e', "export_xlsx", Required = false, HelpText = "Export info to a xlsx file at the location specified")]
        public string? ExportXLSX { get; } = export_xlsx;

        [Option('p', "full_path", Required = false, HelpText = "When displaying paths should they be printed in full", Default = false)]
        public bool FullPath { get; } = full_path;

        [Option('i', "ignore_entries", Required = false, HelpText = "Comma seperated list of directories and files that will be not included in the analysis")]
        public string? IgnoreEntries { get; } = ignore_entries;

        [Option('u', "updates", Required = false, HelpText = "Provides periodic updates about how many entries have been analyized. Value is seconds between updates.")]
        public ulong? Updates { get; } = updates;

        [Option('t', "time", Required = false, HelpText = "Displays the time the program took to run to stdout", Default = false)]
        public bool Time { get; } = time;
    }
}
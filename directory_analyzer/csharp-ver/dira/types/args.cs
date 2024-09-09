using CommandLine;

namespace dira.types
{
    public readonly struct Args(string PathToAnalyze, bool NoRecurse, bool FileInfo, bool FollowSymlinks, bool CountSymlinks, bool Verbose, string? ExportXLSX, bool FullPath, string? IgnoreEntries, ulong? Updates, bool Time)
    {
        [Option('r', "PathToAnalyze", Required = true, HelpText = "Requied path to analyze")]
        public string PathToAnalyze { get; } = PathToAnalyze;

        [Option('n', "NoRecurse", Required = false, HelpText = "Just look at the files in the current directory", Default = false)]
        public bool NoRecurse { get; } = NoRecurse;

        [Option('f', "FileInfo", Required = false, HelpText = "Output info about what file types were found, such as type and size", Default = false)]
        public bool FileInfo { get; } = FileInfo;

        [Option('s', "FollowSymlinks", Required = false, HelpText = "Follow symlinks as if they were a normal object.\nNOTE: Due to the need to track found files and directories to prevent counting the same entry multiple times, this has a noticable performance impact.Due to this unless you need to actually follow symlinks it is recomended to not use this flag.", Default = false)]
        public bool FollowSymlinks { get; } = FollowSymlinks;

        [Option('c', "CountSymlinks", Required = false, HelpText = "Counts symlinks found but does not follow them", Default = false)]
        public bool CountSymlinks { get; } = CountSymlinks;

        [Option('v', "Verbose", Required = false, HelpText = "Print each object as it is found", Default = false)]
        public bool Verbose { get; } = Verbose;

        [Option('e', "ExportXLSX", Required = false, HelpText = "Export info to a xlsx file at the location specified")]
        public string? ExportXLSX { get; } = ExportXLSX;

        [Option('p', "FullPath", Required = false, HelpText = "When displaying paths should they be printed in full", Default = false)]
        public bool FullPath { get; } = FullPath;

        [Option('i', "IgnoreEntries", Required = false, HelpText = "Comma seperated list of directories and files that will be not included in the analysis")]
        public string? IgnoreEntries { get; } = IgnoreEntries;

        [Option('u', "Updates", Required = false, HelpText = "Provides periodic updates about how many entries have been analyized. Value is seconds between updates.")]
        public ulong? Updates { get; } = Updates;

        [Option('t', "Time", Required = false, HelpText = "Displays the time the program took to run to stdout", Default = false)]
        public bool Time { get; } = Time;
    }
}
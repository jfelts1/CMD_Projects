using System.Diagnostics;
using System.Timers;
using dira.types;

namespace dira
{

    public static class A
    {

        public static AnalyzedInfo Analyze(Args args)
        {
            AnalyzedInfo res = SetUpAnalyzedInfo(args);
            SearchDir(args, ref res);
            res.CalculatePercentagesForInfo();
            return res;
        }

        private static void SearchDir(Args args, ref AnalyzedInfo res)
        {
            var dirs_to_analyze = new Stack<string>();
            dirs_to_analyze.Push(args.PathToAnalyze);
            //used to prevent repeatedly counting the same item multiple times while following symlinks
            var found_items = new List<string>();
            var timer = SetUpTimer(args);
            var ignore_these = SetUpIgnoreThese(args);

            while (dirs_to_analyze.TryPop(out var dir))
            {
                var cur_dir = Directory.EnumerateFileSystemEntries(dir);
                foreach (var entry in cur_dir)
                {
                    var path = Path.GetFullPath(entry).TrimEnd(Path.AltDirectorySeparatorChar, Path.DirectorySeparatorChar);
                    var metadata = File.GetAttributes(path);
                    var info = new FileInfo(path);
                    if (ignore_these != null)
                    {
                        if (ignore_these.Any(p => Path.GetFullPath(p).TrimEnd(Path.AltDirectorySeparatorChar, Path.DirectorySeparatorChar) == path))
                        {
                            continue;
                        }
                    }
                    if (metadata.HasFlag(FileAttributes.Directory) && !found_items.Contains(path) && info.LinkTarget == null)
                    {
                        //Directories
                        HandleDirs(args, ref dirs_to_analyze, entry, ref res);
                        if (args.FollowSymlinks)
                        {
                            found_items.Add(path);
                        }
                    }
                    else if (File.Exists(path) && !found_items.Contains(path) && info.LinkTarget == null)
                    {
                        //Files
                        HandleFiles(args, ref res, entry, info);
                        if (args.FollowSymlinks)
                        {
                            found_items.Add(path);
                        }
                    }
                    else if ((args.CountSymlinks || args.FollowSymlinks) && info.LinkTarget != null && !found_items.Contains(path))
                    {
                        //Symbolic links
                        HandleSymlinks(args, ref found_items, entry, ref dirs_to_analyze, ref res, info);
                    }

                }

                if (timer != null)
                {
                    if ((ulong)timer.Elapsed.Seconds > args.Updates)
                    {
                        timer.Reset();
                        PrintUpdate(res.FoundDirs, res.FoundFiles);
                        timer.Start();
                    }
                }
            }

            static void PrintUpdate(uint found_dirs, uint found_files)
            {
                Console.WriteLine("Update: {0} found dirs, {1} found files", found_dirs, found_files);
            }
            static void HandleDirs(Args args, ref Stack<string> dirs_to_analyze, string entry, ref AnalyzedInfo analyzed_info)
            {
                HandleArgs(args, ref dirs_to_analyze, entry);
                analyzed_info.FoundDirs += 1;

                static void HandleArgs(Args args, ref Stack<string> dirs_to_analyze, string entry)
                {
                    if (!args.NoRecurse)
                    {
                        dirs_to_analyze.Push(entry);
                    }
                    if (args.Verbose)
                    {
                        if (args.FullPath)
                        {
                            Console.WriteLine("dir: {0}", Path.GetFullPath(entry).TrimEnd(Path.AltDirectorySeparatorChar, Path.DirectorySeparatorChar));
                        }
                        else
                        {
                            Console.WriteLine("dir:{0}", entry);
                        }
                    }
                }
            }

            static void HandleFiles(Args args, ref AnalyzedInfo analyzed_info, string entry, FileInfo info)
            {
                HandleArgs(args, ref analyzed_info, entry, info);
                analyzed_info.FoundFiles += 1;
                analyzed_info.TotalBytes += (ulong)info.Length;

                static void HandleArgs(Args args, ref AnalyzedInfo analyzed_info, string entry, FileInfo info)
                {
                    if (args.Verbose)
                    {
                        if (args.FullPath)
                        {
                            Console.WriteLine("file:{0}", Path.GetFullPath(entry));
                        }
                        else
                        {
                            Console.WriteLine("file: {0}", entry);
                        }
                    }

                    if (analyzed_info.FileInfo != null)
                    {
                        switch (Path.GetExtension(entry))
                        {
                            case string ext when ext != "":
                                //Remove leaving . for consistancy with the rust version
                                AddFileInfoToMap(args, new FileExtension(ext[1..]), ref analyzed_info, entry, info);
                                break;
                            case "":
                                //Still want to keep info about files without extensions
                                AddFileInfoToMap(args, new FileExtension(""), ref analyzed_info, entry, info);
                                break;
                        }
                    }
                }
            }

            static void HandleSymlinks(Args args, ref List<string> found_items, string entry, ref Stack<string> dirs_to_analyze, ref AnalyzedInfo analyzed_info, FileInfo f_info)
            {
                Trace.Assert(f_info.LinkTarget != null);
                var path = Path.GetFullPath(f_info.LinkTarget).TrimEnd(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar);
                var metadata = File.GetAttributes(path);
                var info = new FileInfo(path);
                if (args.FollowSymlinks)
                {
                    //don't look at entries that have been seen before
                    //prevents following symlink loops and counting entries multiple times
                    if (!found_items.Contains(path))
                    {
                        if (metadata.HasFlag(FileAttributes.Directory))
                        {
                            HandleDirs(args, ref dirs_to_analyze, entry, ref analyzed_info);
                        }
                        else if (File.Exists(path))
                        {
                            HandleFiles(args, ref analyzed_info, entry, info);
                        }
                        found_items.Add(path);
                    }
                }
                if (args.CountSymlinks && analyzed_info.FoundSymlinks is SymlinkInfo symlink)
                {
                    //Count the found symlinks here since a symlink can be new but point to a entry that
                    //has already been seen before. We still want to count the symlink as found though
                    if (metadata.HasFlag(FileAttributes.Directory))
                    {
                        var sym = new SymlinkInfo();
                        sym.SetDirSymlinks(symlink.GetDirSymlinks() + 1);
                        sym.SetFileSymlinks(symlink.GetFileSymlinks());
                        analyzed_info.FoundSymlinks = sym;
                    }
                    else if (File.Exists(path))
                    {
                        var sym = new SymlinkInfo();
                        sym.SetDirSymlinks(symlink.GetDirSymlinks());
                        sym.SetFileSymlinks(symlink.GetFileSymlinks() + 1);
                        analyzed_info.FoundSymlinks = sym;
                    }

                }

            }
        }

        /// <summary>
        /// Must make sure AnalyzedInfo.FileInfo is not null before calling.
        /// </summary>
        /// <param name="args"></param>
        /// <param name="extension"></param>
        /// <param name="info"></param>
        /// <param name="entry"></param>
        /// <param name="metadata"></param>
        private static void AddFileInfoToMap(Args args, FileExtension extension, ref AnalyzedInfo info, string entry, FileInfo metadata)
        {
            Trace.Assert(info.FileInfo != null);
            if (!info.FileInfo.TryGetValue(extension, out FileTypeInfo? value))
            {
                var file_type_info = new FileTypeInfo(0, new FileTypeInfoRecords(), new FileTypeInfoRecords("", ulong.MaxValue), 0, null, null);
                value = file_type_info;
                info.FileInfo.Add(extension, value);
            }

            value.NumFiles += 1;
            value.SizeInBytes += (ulong)metadata.Length;
            string? path = null;
            if (args.FullPath)
            {
                path = Path.GetFullPath(entry).TrimEnd(Path.AltDirectorySeparatorChar, Path.DirectorySeparatorChar);
            }
            else
            {
                path = entry;
            }
            if ((ulong)metadata.Length > value.LargestFile.Size)
            {
                value.LargestFile = new FileTypeInfoRecords(path, (ulong)metadata.Length);
            }
            if ((ulong)metadata.Length < value.SmallestFile.Size)
            {
                value.SmallestFile = new FileTypeInfoRecords(path, (ulong)metadata.Length);
            }

        }

        private static List<string>? SetUpIgnoreThese(Args args)
        {
            List<string>? ignore_these = null;
            if (args.IgnoreEntries != null)
            {
                ignore_these = new List<string>(args.IgnoreEntries.Split(',').Select(s => s.Trim()));
                foreach (var p in ignore_these)
                {
                    if (!Path.Exists(p))
                    {
                        Console.SetOut(Console.Error);
                        Console.WriteLine("WARNING: Can't ignore \"{0}\" because it doesn't exist", p);
                        Console.SetOut(Console.Out);
                    }
                }
            }
            return ignore_these;
        }

        private static Stopwatch? SetUpTimer(Args args)
        {
            if (args.Updates != null)
            {
                var timer = new Stopwatch();
                timer.Start();
                return timer;
            }

            return null;
        }

        /// <summary>
        /// Configures AnalyzedInfo based on the Args given
        /// </summary>
        /// <param name="args"></param>
        /// <returns></returns>
        private static AnalyzedInfo SetUpAnalyzedInfo(Args args)
        {
            AnalyzedInfo res = new() { FoundFiles = 0, FoundDirs = 0, TotalBytes = 0, FoundSymlinks = null, FileInfo = null };
            if (args.FileInfo)
            {
                res.FileInfo = [];
            }
            if (args.CountSymlinks)
            {
                res.FoundSymlinks = new SymlinkInfo();
            }
            return res;
        }
    }
}
namespace UnitTests;
using Xunit;
using dira.types;
using dira;

public class AnalyzeUnitTests
{
    //Because paths are strings not a dedicated type 
#if WINDOWS
    const string ROOT_DIR = @"..\..\..\..\";
    const string TEST_DIR = ROOT_DIR + @"..\test\";
#elif LINUX || MAC
    const string ROOT_DIR = @"../../../../";
    const string TEST_DIR = ROOT_DIR + @"../test/";
#endif
    [Fact]
    public void AnalyzeDefaultSettings()
    {
        Args test_args = new(TEST_DIR,
            false,//no_recurse
           false,//file_info
           false,//follow_symlinks
           true,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           null,//ignore_entries
           null,//updates
           false//time
           );
        AnalyzedInfo res = A.Analyze(test_args);
        SymlinkInfo sym = new();
        sym.SetDirSymlinks(1);
        sym.SetFileSymlinks(1);
        AnalyzedInfo expected = new()
        { FoundDirs = 4, FoundFiles = 7, FoundSymlinks = sym, FileInfo = null, TotalBytes = 432 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeNoRecurse()
    {
        Args test_args = new(TEST_DIR,
            true,//no_recurse
           false,//file_info
           false,//follow_symlinks
           false,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           null,//ignore_entries
           null,//updates
           false//time
           );
        AnalyzedInfo res = A.Analyze(test_args);
        AnalyzedInfo expected = new() { FoundDirs = 2, FoundFiles = 4, FoundSymlinks = null, FileInfo = null, TotalBytes = 407 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeFileInfo()
    {
        Args test_args = new(TEST_DIR,
            false,//no_recurse
           true,//file_info
           false,//follow_symlinks
           false,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           null,//ignore_entries
           null,//updates
           false//time
           );
        var res = A.Analyze(test_args);
        var hash_map = new Dictionary<FileExtension, FileTypeInfo>();

        var txt = new FileExtension("txt");
        var rtf = new FileExtension("rtf");
        var zip = new FileExtension("zip");
        hash_map.Add(txt, new FileTypeInfo(1, new FileTypeInfoRecords(), new FileTypeInfoRecords(), 9, null, null));//file1.txt
        //file2.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 14;
        hash_map[txt].LargestFile = new FileTypeInfoRecords(TEST_DIR + "file2.txt", 14);

        //folder1/file5.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 9;

        //folder2/file6.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 7;
        hash_map[txt].SmallestFile = new FileTypeInfoRecords(TEST_DIR + "folder2" + Path.DirectorySeparatorChar + "file6.txt", 7);

        //folder2/folder3/folder4/deepfile1.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 9;

        hash_map.Add(rtf, new FileTypeInfo(1, new FileTypeInfoRecords(TEST_DIR + "file3.rtf", 196), new FileTypeInfoRecords(TEST_DIR + "file3.rtf", 196), 196, null, null));//file3.rtf
        hash_map.Add(zip, new FileTypeInfo(1, new FileTypeInfoRecords(TEST_DIR + "file4.zip", 188), new FileTypeInfoRecords(TEST_DIR + "file4.zip", 188), 188, null, null));//file4.zip

        (var total_files, var total_bytes) = GetTotalFilesAndBytesFromMap(hash_map);
        foreach (var info in hash_map)
        {
            info.Value.CalculatePercentages(total_bytes, total_files);
        }

        AnalyzedInfo expected = new() { FoundDirs = 4, FoundFiles = 7, FoundSymlinks = null, FileInfo = hash_map, TotalBytes = 432 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeFollowSymlink()
    {
        Args test_args = new(TEST_DIR,
            false,//no_recurse
           false,//file_info
           true,//follow_symlinks
           true,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           null,//ignore_entries
           null,//updates
           false//time
           );

        var res = A.Analyze(test_args);
        var sym = new SymlinkInfo();
        sym.SetDirSymlinks(1);
        sym.SetFileSymlinks(1);
        var expected = new AnalyzedInfo() { FoundDirs = 6, FoundFiles = 8, FileInfo = null, FoundSymlinks = sym, TotalBytes = 432 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeIgnoreEntries()
    {
        var str1 = TEST_DIR + "folder2" + Path.DirectorySeparatorChar + "folder3";
        var str2 = TEST_DIR + "file4.zip";
        var format = "{0}, {1}";
        var ignore_entries = String.Format(format, str1, str2);
        Args test_args = new(TEST_DIR,
            false,//no_recurse
           false,//file_info
           false,//follow_symlinks
           false,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           ignore_entries,//ignore_entries
           null,//updates
           false//time
           );
        var res = A.Analyze(test_args);
        var expected = new AnalyzedInfo() { FoundDirs = 2, FoundFiles = 5, FoundSymlinks = null, FileInfo = null, TotalBytes = 235 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeIgnoreEntriesFileInfo()
    {
        var str1 = TEST_DIR + "folder2" + Path.DirectorySeparatorChar + "folder3";
        var str2 = TEST_DIR + "file4.zip";
        var format = "{0}, {1}";
        var ignore_entries = String.Format(format, str1, str2);
        Args test_args = new(TEST_DIR,
            false,//no_recurse
           true,//file_info
           false,//follow_symlinks
           false,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           ignore_entries,//ignore_entries
           null,//updates
           false//time
           );
        var res = A.Analyze(test_args);
        var hash_map = new Dictionary<FileExtension, FileTypeInfo>();

        var txt = new FileExtension("txt");
        var rtf = new FileExtension("rtf");
        hash_map.Add(txt, new FileTypeInfo(1, new FileTypeInfoRecords(), new FileTypeInfoRecords(), 9, null, null));//file1.txt
        //file2.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 14;
        hash_map[txt].LargestFile = new FileTypeInfoRecords(TEST_DIR + "file2.txt", 14);

        //folder1/file5.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 9;

        //folder2/file6.txt
        hash_map[txt].NumFiles += 1;
        hash_map[txt].SizeInBytes += 7;
        hash_map[txt].SmallestFile = new FileTypeInfoRecords(TEST_DIR + "folder2" + Path.DirectorySeparatorChar + "file6.txt", 7);

        hash_map.Add(rtf, new FileTypeInfo(1, new FileTypeInfoRecords(TEST_DIR + "file3.rtf", 196), new FileTypeInfoRecords(TEST_DIR + "file3.rtf", 196), 196, null, null));//file3.rtf

        (var total_files, var total_bytes) = GetTotalFilesAndBytesFromMap(hash_map);
        foreach (var info in hash_map)
        {
            info.Value.CalculatePercentages(total_bytes, total_files);
        }

        var expected = new AnalyzedInfo() { FoundDirs = 2, FoundFiles = 5, FoundSymlinks = null, FileInfo = hash_map, TotalBytes = 235 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeSymlinkLoop()
    {
        var test_dir = ROOT_DIR + ".." + Path.DirectorySeparatorChar + "test_symlink_loop";
        Args test_args = new(test_dir,
            false,//no_recurse
           false,//file_info
           true,//follow_symlinks
           true,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           null,//ignore_entries
           null,//updates
           false//time
           );
        var res = A.Analyze(test_args);
        var sym = new SymlinkInfo();
        sym.SetDirSymlinks(2);
        var expected = new AnalyzedInfo() { FoundDirs = 2, FoundFiles = 2, FoundSymlinks = sym, FileInfo = null, TotalBytes = 21 };
        Assert.Equal(expected, res);
    }

    [Fact]
    public void AnalyzeFollowSymlinkNoCountSymlinks()
    {
        Args test_args = new(TEST_DIR,
            false,//no_recurse
           false,//file_info
           true,//follow_symlinks
           false,//count_symlinks
           false,//verbose
           null,//export_xlsx
           false,//full_path
           null,//ignore_entries
           null,//updates
           false//time
           );
        var res = A.Analyze(test_args);
        var expected = new AnalyzedInfo() { FoundDirs = 6, FoundFiles = 8, FoundSymlinks = null, FileInfo = null, TotalBytes = 432 };
        Assert.Equal(expected, res);
    }

    public static (uint, ulong) GetTotalFilesAndBytesFromMap(Dictionary<FileExtension, FileTypeInfo> map)
    {
        uint total_files = 0;
        ulong total_bytes = 0;

        foreach (var info in map)
        {
            total_files += info.Value.NumFiles;
            total_bytes += info.Value.SizeInBytes;
        }

        return (total_files, total_bytes);
    }
}

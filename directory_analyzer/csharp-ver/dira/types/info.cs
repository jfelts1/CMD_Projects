using System.Diagnostics.CodeAnalysis;

namespace dira.types
{

    public class AnalyzedInfo
    {
        public AnalyzedInfo()
        {
        }

        /// <summary>
        /// Count of directories found during the analysis
        /// </summary>
        public required uint FoundDirs { get; set; }

        /// <summary>
        /// Count of files found during the analysis
        /// </summary>
        public required uint FoundFiles { get; set; }

        /// <summary>
        /// Info about symlinks found during the analysis
        /// </summary>
        public SymlinkInfo? FoundSymlinks { get; set; }

        /// <summary>
        /// Info about files grouped by file type
        /// </summary>
        public Dictionary<FileExtension, FileTypeInfo>? FileInfo { get; set; }

        /// <summary>
        /// How many bytes taken up by all the files
        /// </summary>
        public required ulong TotalBytes { get; set; }


        /// <summary>
        /// Calculates the the percent of total files and percent of total size on disk for each type of file,if file info is gathered.
        /// </summary>
        public void CalculatePercentagesForInfo()
        {
            if (FileInfo != null)
            {
                foreach (var info in FileInfo)
                {
                    info.Value.CalculatePercentages(TotalBytes, FoundFiles);
                }
            }
        }

        public override bool Equals(object? obj)
        {
            if (obj == null || obj is not AnalyzedInfo)
            {
                return false;
            }
            var other = (AnalyzedInfo)obj;
            bool dicts = true;
            if (FileInfo != null && other.FileInfo != null)
            {
                dicts = other.FileInfo.All(kv =>
                {
                    if (FileInfo.TryGetValue(kv.Key, out FileTypeInfo? value))
                    {
                        return value == kv.Value;
                    }
                    else
                    {
                        return false;
                    }
                });
            }
            bool syms = this.FoundSymlinks == other.FoundSymlinks;

            return this.FoundDirs == other.FoundDirs && this.FoundFiles == other.FoundFiles && this.TotalBytes == other.TotalBytes && syms && dicts;
        }

        public static bool operator ==(AnalyzedInfo left, AnalyzedInfo right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(AnalyzedInfo left, AnalyzedInfo right)
        {
            return !(left == right);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(FoundDirs, FoundFiles, FoundSymlinks, FileInfo, TotalBytes);
        }
    }

    public readonly struct FileExtension(string ext)
    {
        public string Ext { get; } = ext;
    }

    public class FileTypeInfo(uint num_files, FileTypeInfoRecords largest_file, FileTypeInfoRecords smallest_file, ulong size_in_bytes, float? percent_of_total_files, float? percent_of_total_size)
    {
        /// <summary>
        /// Number of files of this type
        /// </summary>
        public uint NumFiles { get; set; } = num_files;

        public FileTypeInfoRecords LargestFile { get; set; } = largest_file;

        public FileTypeInfoRecords SmallestFile { get; set; } = smallest_file;

        /// <summary>
        /// Total size of all files of this type
        /// </summary>
        public ulong SizeInBytes { get; set; } = size_in_bytes;

        public float? PercentOfTotalFiles { get; private set; } = percent_of_total_files;

        public float? PercentOfTotalSize { get; private set; } = percent_of_total_size;

        /// <summary>
        /// Calculates PercentOfTotalFiles and PercentOfTotalSize
        /// </summary>
        /// <param name="total_bytes"></param>
        /// <param name="total_files"></param>
        public void CalculatePercentages(ulong total_bytes, uint total_files)
        {
            PercentOfTotalFiles = (float)NumFiles / (float)total_files;
            PercentOfTotalSize = (float)SizeInBytes / (float)total_bytes;
        }

        public override bool Equals(object? obj)
        {
            if (obj != null && obj is FileTypeInfo other)
            {
                return this.NumFiles == other.NumFiles && this.LargestFile == other.LargestFile && this.SmallestFile == other.SmallestFile && this.SizeInBytes == other.SizeInBytes && this.PercentOfTotalFiles == other.PercentOfTotalFiles && this.PercentOfTotalSize == other.PercentOfTotalSize;
            }
            return false;
        }

        public static bool operator ==(FileTypeInfo left, FileTypeInfo right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(FileTypeInfo left, FileTypeInfo right)
        {
            return !(left == right);
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(NumFiles, LargestFile, SmallestFile, SizeInBytes, PercentOfTotalFiles, PercentOfTotalSize);
        }
    }

    /// <summary>
    /// This is for holding info about specific notable files
    /// </summary>
    public readonly struct FileTypeInfoRecords(string path, ulong size)
    {
        /// <summary>
        /// Path to the file
        /// </summary>
        public string Path { get; } = path;

        /// <summary>
        /// The size of the file
        /// </summary>
        public ulong Size { get; } = size;

        public override bool Equals([NotNullWhen(true)] object? obj)
        {
            if (obj != null && obj is FileTypeInfoRecords other)
            {

                return this.Path == other.Path && this.Size == other.Size;
            }
            return false;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Path, Size);
        }

        public static bool operator ==(FileTypeInfoRecords left, FileTypeInfoRecords right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(FileTypeInfoRecords left, FileTypeInfoRecords right)
        {
            return !(left == right);
        }
    }

    public struct SymlinkInfo
    {

        public uint FoundSymlinks { get; private set; }

        /// <summary>
        /// Number of symlinks that point to files
        /// </summary>
        private uint fileSymlinks;

        /// <summary>
        /// Number of symlinks that point to files
        /// </summary>
        /// <returns></returns>
        public readonly uint GetFileSymlinks()
        {
            return fileSymlinks;
        }

        public void SetFileSymlinks(uint value)
        {
            fileSymlinks = value;
            FoundSymlinks = fileSymlinks + dirSymlinks;
        }

        /// <summary>
        /// Number of symlinks that point to directories
        /// </summary>
        private uint dirSymlinks;

        /// <summary>
        /// Number of symlinks that point to directories
        /// </summary>
        /// <returns></returns>
        public readonly uint GetDirSymlinks()
        {
            return dirSymlinks;
        }

        public void SetDirSymlinks(uint value)
        {
            dirSymlinks = value;
            FoundSymlinks = fileSymlinks + dirSymlinks;
        }

        public override readonly bool Equals([NotNullWhen(true)] object? obj)
        {
            if (obj != null && obj is SymlinkInfo other)
            {
                return this.dirSymlinks == other.dirSymlinks && this.fileSymlinks == other.fileSymlinks && this.FoundSymlinks == other.FoundSymlinks;
            }
            return false;
        }

        public override readonly int GetHashCode()
        {
            return HashCode.Combine(dirSymlinks, fileSymlinks, FoundSymlinks);
        }

        public static bool operator ==(SymlinkInfo left, SymlinkInfo right)
        {
            return left.Equals(right);
        }

        public static bool operator !=(SymlinkInfo left, SymlinkInfo right)
        {
            return !(left == right);
        }
    }
}
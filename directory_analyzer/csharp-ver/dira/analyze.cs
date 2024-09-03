using dira.types;

namespace dira
{

    public static class A
    {

        public static AnalyzedInfo Analyze(Args args)
        {
            AnalyzedInfo res = SetUpAnalyzedInfo(args);
            //TODO: search directories

            return res;
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
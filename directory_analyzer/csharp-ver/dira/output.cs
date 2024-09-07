
using dira.types;

namespace dira
{

    public static class B
    {
        public static string Output(Args args, AnalyzedInfo analyzedInfo)
        {
            if (args.ExportXLSX != null)
            {
                throw new NotImplementedException();
                //TODO: Write to xlsx file

                string format = "Info saved to {}";
                return String.Format(format, args.ExportXLSX);

            }
            else
            {
                string format = "{}";
                return String.Format(format, analyzedInfo);
            }
        }
    }
}
using System.Diagnostics;
using CommandLine;
using dira.types;

namespace dira
{
    class Dira
    {
        static void Main(string[] args)
        {
            var result = Parser.Default.ParseArguments<Args>(args).WithParsed(args =>
            {
                try
                {
                    Stopwatch? start_time = null;
                    if (args.Time)
                    {
                        start_time = new Stopwatch();

                        start_time.Start();
                    }
                    var o = B.Output(args, A.Analyze(args));
                    Console.WriteLine(o);
                    if (start_time != null)
                    {
                        start_time.Stop();
                        var ts = start_time.Elapsed;
                        Console.WriteLine("Took {0}.{1:#} seconds", ts.Seconds, ts.Milliseconds / 10);
                    }
                }
                catch (Exception e)
                {
                    Console.SetOut(Console.Error);
                    Console.WriteLine("{0}", e);
                }
            }).WithNotParsed(e =>
            {
                Console.SetOut(Console.Error);
                Console.WriteLine("{0}", e);
            });


        }

    }
}
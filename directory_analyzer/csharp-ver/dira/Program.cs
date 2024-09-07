using System;
using CommandLine;
using dira.types;

namespace dira
{
    class Dira
    {
        static void Main(string[] args)
        {
            //TODO: Start timer
            var parser = new Parser(with => with.EnableDashDash = true);
            var result = parser.ParseArguments<Args>(args).WithParsed<Args>(args =>
            {
                try
                {
                    var o = B.Output(args, A.Analyze(args));
                    Console.WriteLine(o);
                }
                catch (Exception e)
                {
                    Console.SetOut(Console.Error);
                    Console.WriteLine("{0}", e);
                }

            }).WithNotParsed<Args>(e => { });
        }

    }
}
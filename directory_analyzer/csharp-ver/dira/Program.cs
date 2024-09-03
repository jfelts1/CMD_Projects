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
            var result = parser.ParseArguments<Args>(args).WithParsed<Args>(args => { 
                var info = A.Analyze(args);
                var o = B.Output(args,info);
                System.Console.WriteLine(o);
                
            }).WithNotParsed<Args>(e => { });
        }

    }
}
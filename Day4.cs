using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net;
using System.Threading;

namespace adventofcode2017
{
    class Day4
    {
        internal static bool IsValid(string password)
        {
            var raw = password.Split(' ');
            return raw.Length == raw.ToHashSet().Count;                    
        }

        internal static void Run1()
        {
            var input = File.ReadAllLines("input/Day4.txt");
            var count = 0;
            foreach (var password in input)
            {
                if (IsValid(password))
                    count++;
            }

            Console.WriteLine(count);
        }        

        internal static bool IsValid2(string password)
        {
            var raw = password.Split(' ');
            return raw.Length == raw.Select(x => string.Concat(x.OrderBy(c => c)))
                                    .ToHashSet()
                                    .Count;                    
        }

        internal static void Run2()
        {
            var input = File.ReadAllLines("input/Day4.txt");
            var count = 0;
            foreach (var password in input)
            {
                if (IsValid2(password))
                    count++;
            }

            Console.WriteLine(count);
            Console.WriteLine(IsValid2("oiii ioii iioi iiio"));
        }        

    }

}
using System;
using System.IO;

namespace adventofcode2017
{
    class Day5
    {
        internal static void Run1()
        {
            var input = File.ReadAllLines("input/Day5.txt");
            //var input = new string[] { "0" , "3", "0", "1", "-3" };
            var a = new int[input.Length];
            
            for (int i = 0; i < input.Length; i++)
                a[i] = int.Parse(input[i]);

            var pos = 0;
            var count = 0;

            while (pos >= 0 && pos < a.Length)
            {
                a[pos]++;
                pos = pos + a[pos] - 1;
                count++;
            }

            Console.WriteLine(count);
        }
    }
}
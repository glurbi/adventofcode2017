using System;
using System.Linq;

namespace adventofcode2017
{
    class Day3
    {
        internal static void Run()
        {
            var (x, y) = (0, 0);
            var (minX, minY, maxX, maxY) = (0, 0, 0, 0);

            var pos = 1;
            const int goal = 265149;

            while (pos < goal)
            {
                while (pos < goal && x <= maxX)
                {
                    x++;
                    pos++;
                }

                maxX = x;

                while (pos < goal && y <= maxY)
                {
                    y++;
                    pos++;
                }

                maxY = y;

                while (pos < goal && x >= minX)
                {
                    x--;
                    pos++;
                }

                minX = x;

                while (pos < goal && y >= minY)
                {
                    y--;
                    pos++;
                }

                minY = y;
            }

            Console.WriteLine((x, y) + " " + (Math.Abs(x)+Math.Abs(y)));
        }
    }
}

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;

namespace adventofcode2017
{
    class Day3
    {
        internal static void Run1()
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

        class Memory
        {
            Dictionary<Tuple<int, int>, int> dic = new Dictionary<Tuple<int, int>, int>();

            internal Memory()
            {
                this[0,0] = 1;
            }

            int this[int x, int y]
            {
                get
                {
                    var xy = (x, y).ToTuple();
                    return (dic.ContainsKey(xy)) ? dic[xy] : 0;
                }

                set
                {
                    var xy = (x, y).ToTuple();
                    dic[xy] = value;
                }
            }

            int Compute(int x, int y)
            {
                return this[x-1, y+1] + this[x-0, y+1] + this[x+1, y+1]
                     + this[x-1, y-0] +                  this[x+1, y+0]
                     + this[x-1, y-1] + this[x-0, y-1] + this[x+1, y-1];
            }

            internal IEnumerable<int> Values()
            {
                yield return 1;

                var (x, y) = (0, 0);
                var (minX, minY, maxX, maxY) = (0, 0, 0, 0);

                while (true)
                {
                    while (x <= maxX)
                    {
                        x++;
                        this[x, y] = Compute(x, y);
                        yield return this[x, y];
                    }

                    maxX = x;

                    while (y <= maxY)
                    {
                        y++;
                        this[x, y] = Compute(x, y);
                        yield return this[x, y];
                    }

                    maxY = y;

                    while (x >= minX)
                    {
                        x--;
                        this[x, y] = Compute(x, y);
                        yield return this[x, y];
                    }

                    minX = x;

                    while (y >= minY)
                    {
                        y--;
                        this[x, y] = Compute(x, y);
                        yield return this[x, y];
                    }

                    minY = y;
                }
            }
        }

        internal static void Run2()
        {
            var mem = new Memory();

            foreach (int i in mem.Values())
            {
                Console.WriteLine(i);

                if (i > 265149)
                    break;
            }
        }
    }
}

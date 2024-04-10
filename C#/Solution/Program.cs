using Solution.year_2015.day_1;
using Solution.year_2015.day_3;
using Solution.year_2015.day_4;
using Solution.year_2015.day_5;
using Solution.year_2015.day_6;

namespace Solution
{
    public static class Solution
    {
        public static void Main(string[] args)
        {
            BatchWrite(
                Day6Part2.Run()
            );
        }
        public static void BatchWrite(params object[] toWrite)
        {
            foreach (object tw in toWrite)
                Console.WriteLine($"{tw}");
        }
    }
}
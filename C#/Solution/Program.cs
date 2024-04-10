using Solution.year_2015.day_1;

namespace Solution
{
    public static class Solution
    {
        public static void Main(string[] args)
        {
            BatchWrite(
                );
        }
        public static void BatchWrite(params object[] toWrite)
        {
            foreach (object tw in toWrite)
                Console.WriteLine(tw.ToString());
        }
    }
}
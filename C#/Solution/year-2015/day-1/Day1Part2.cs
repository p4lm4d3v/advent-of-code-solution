using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Solution.year_2015.day_1
{
    public static class Day1Part2
    {
        public static int Proccess(string input)
        {
            int currentFloor = 0;
            int counter = 1;
            for (int i = 0; i < input.Length; i++)
            {
                char s = input[i];
                if (s == '(') currentFloor++;
                else if (s == ')') currentFloor--;

                if (currentFloor < 0)
                    break;
                counter++;
            }
            return counter;
        }
    }
    public static class Day1Part2Test
    {
        public static int Test1() => Day1Part2.Proccess(")");
        public static int Test2() => Day1Part2.Proccess("()())");
    }
}

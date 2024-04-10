namespace Solution.year_2015.day_5
{
  public static class Day5Part2
  {
    public static bool ContainTwoPairs(string input)
    {
      for (int i = 0; i < input.Length - 1; i++)
      {
        string iPair = $"{input[i]}{input[i + 1]}";
        for (int j = 0; j < input.Length - 1; j++)
        {
          string jPair = $"{input[j]}{input[j + 1]}";
          if (j != i && j + 1 != i && i + 1 != j)
            if (iPair == jPair)
              return true;
        }
      }
      return false;
    }
    public static bool LetterInMiddle(string input)
    {
      for (int i = 0; i < input.Length - 2; i++)
        if (input[i] == input[i + 2])
          return true;
      return false;
    }
    static bool IsNice(string input)
    {
      bool containTwoPairs = ContainTwoPairs(input);
      bool letterInMiddle = LetterInMiddle(input);
      return containTwoPairs && letterInMiddle;
    }
    public static int Process(string input)
    {
      string[] lines = input.Split('\n').Select(x => x.Trim()).ToArray();
      int counter = 0;
      foreach (string line in lines)
        if (Day5Part2.IsNice(line))
          counter++;
      return counter;
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-5/input.txt");
      return Day5Part1.Process(input);
    }
  }
  public static class Day5Part2Test
  {
    public static int TestProcess1() => Day5Part2.Process("qjhvhtzxzqqjkmpb");
    public static int TestProcess2() => Day5Part2.Process("xxyxx");
    public static int TestProcess3() => Day5Part2.Process("uurcxstgmygtbstg");
    public static int TestProcess4() => Day5Part2.Process("ieodomkazucvgmuy");
    public static bool TestContainTwoPairs1() => Day5Part2.ContainTwoPairs("qjhvhtzxzqqjkmpb");
    public static bool TestContainTwoPairs2() => Day5Part2.ContainTwoPairs("xxyxx");
    public static bool TestContainTwoPairs3() => Day5Part2.ContainTwoPairs("uurcxstgmygtbstg");
    public static bool TestContainTwoPairs4() => Day5Part2.ContainTwoPairs("ieodomkazucvgmuy");
    public static bool TestContainTwoPairs5() => Day5Part2.ContainTwoPairs("aaa");
    public static bool TestLetterInMiddle1() => Day5Part2.LetterInMiddle("qjhvhtzxzqqjkmpb");
    public static bool TestLetterInMiddle2() => Day5Part2.LetterInMiddle("xxyxx");
    public static bool TestLetterInMiddle3() => Day5Part2.LetterInMiddle("uurcxstgmygtbstg");
    public static bool TestLetterInMiddle4() => Day5Part2.LetterInMiddle("ieodomkazucvgmuy");
  }
}
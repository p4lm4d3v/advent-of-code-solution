namespace Solution.year_2015.day_5
{
  public static class Day5Part1
  {
    static bool AtLeast3Vowels(string input)
    {
      char[] allVowels = new char[] { 'a', 'e', 'i', 'o', 'u' };
      List<char> containedVowels = new List<char>();
      for (int i = 0; i < input.Length; i++)
        if (allVowels.Contains(input[i]))
          containedVowels.Add(input[i]);
      return containedVowels.Count >= 3;
    }
    static bool TwoInARow(string input)
    {
      for (int i = 0; i < input.Length - 1; i++)
        if (input[i] == input[i + 1])
          return true;
      return false;
    }
    static bool DoenstContain(string input)
    {
      bool ab = !input.Contains("ab");
      bool cd = !input.Contains("cd");
      bool pq = !input.Contains("pq");
      bool xy = !input.Contains("xy");
      return ab && cd && pq && xy;
    }
    static bool IsNice(string input)
    {
      bool atLeast3Vowels = Day5Part1.AtLeast3Vowels(input);
      bool twoInARow = Day5Part1.TwoInARow(input);
      bool doesntContain = Day5Part1.DoenstContain(input);
      return atLeast3Vowels && twoInARow && doesntContain;
    }
    public static int Process(string input)
    {
      string[] lines = input.Split('\n').Select(x => x.Trim()).ToArray();
      int counter = 0;
      foreach (string line in lines)
        if (Day5Part1.IsNice(line))
          counter++;
      return counter;
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-5/input.txt");
      return Day5Part1.Process(input);
    }
  }
  public static class Day5Part1Test
  {
    public static int Test1() => Day5Part1.Process("ugknbfddgicrmopn");
    public static int Test2() => Day5Part1.Process("aaa");
    public static int Test3() => Day5Part1.Process("jchzalrnumimnmhp");
    public static int Test4() => Day5Part1.Process("haegwjzuvuyypxyu");
    public static int Test5() => Day5Part1.Process("dvszwmarrgswjxmb");
  }
}
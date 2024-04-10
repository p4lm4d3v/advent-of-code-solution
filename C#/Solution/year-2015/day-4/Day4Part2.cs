namespace Solution.year_2015.day_4
{
  public static class Day4Part2
  {
    static string MD5(string input)
    {
      using (System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create())
      {
        byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
        byte[] hashBytes = md5.ComputeHash(inputBytes);
        return Convert.ToHexString(hashBytes).ToLower();
      }
    }
    public static int Process(string input)
    {
      input = input.Trim();
      int counter = 60000;
      while (true)
      {
        string hash = Day4Part2.MD5($"{input}{counter}");
        if (hash.StartsWith("000000"))
          break;
        counter++;
      }
      return counter;
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-4/input.txt");
      return Day4Part2.Process(input);
    }
  }
  public static class Day4Part2Test { }
}

namespace Solution.year_2015.day_3
{
  public static class Day3Part1
  {
    public static int Process(string input)
    {
      HashSet<(int, int)> visited = [(0, 0)];

      string[] lines = input.Split('\n').Select(x => x.Trim()).ToArray();
      for (int i = 0; i < lines.Length; i++)
      {
        string line = lines[i];
        int horizontal = 0;
        int vertical = 0;
        for (int j = 0; j < line.Length; j++)
        {
          char c = line[j];
          switch (c)
          {
            case '^': vertical++; break;
            case '>': horizontal++; break;
            case 'v': vertical--; break;
            case '<': horizontal--; break;
          }
          visited.Add((horizontal, vertical));
        }
      }

      return visited.Count;
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-3/input.txt");
      return Day3Part1.Process(input);
    }
  }
  public static class Day3Part1Test
  {
    public static int Test1() => Day3Part1.Process(">");
    public static int Test2() => Day3Part1.Process("^>v<");
    public static int Test3() => Day3Part1.Process("^v^v^v^v^v");
  }
}
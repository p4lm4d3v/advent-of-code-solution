namespace Solution.year_2015.day_3
{
  public static class Day3Part2
  {
    static (int, int) ParseDirection(char c)
    {
      switch (c)
      {
        case '^': return (0, 1);
        case '>': return (1, 0);
        case 'v': return (0, -1);
        case '<': return (-1, 0);
      }
      throw new Exception("Error");
    }
    public static int Proccess(string input)
    {
      HashSet<(int, int)> visited = [(0, 0)];

      string[] lines = input.Split('\n').Select(x => x.Trim()).ToArray();
      for (int i = 0; i < lines.Length; i++)
      {
        string line = lines[i];

        (int, int) normal = (0, 0);
        (int, int) robot = (0, 0);

        for (int j = 0; j < line.Length; j++)
        {
          char c = line[j];

          if ((j + 1) % 2 != 0)
          {
            var (x, y) = ParseDirection(c);
            normal.Item1 += x;
            normal.Item2 += y;
          }
          else
          {
            var (x, y) = ParseDirection(c);
            robot.Item1 += x;
            robot.Item2 += y;
          }
          visited.Add(normal);
          visited.Add(robot);
        }
      }
      return visited.Count;
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-3/input.txt");
      return Day3Part2.Proccess(input);
    }
  }
  public static class Day3Part2Test
  {
    public static int Test1() => Day3Part2.Proccess("^v");
    public static int Test2() => Day3Part2.Proccess("^>v<");
    public static int Test3() => Day3Part2.Proccess("^v^v^v^v^v");
  }
}
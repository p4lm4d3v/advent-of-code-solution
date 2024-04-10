
using System.Security.Cryptography.X509Certificates;

namespace Solution.year_2015.day_6
{
  class Lights2
  {
    private delegate void ModifyLightFunction(int x, int y);
    private int[,] grid;
    public int brighteness;

    public Lights2(int size)
    {
      grid = new int[size, size];
      brighteness = 0;
    }

    public int Total()
    {
      int sum = 0;
      for (int i = 0; i < grid.GetLength(0); i++)
        for (int j = 0; j < grid.GetLength(1); j++)
          sum += grid[i, j];
      return sum;
    }

    public void ToggleRange(int x1, int y1, int x2, int y2) => ApplyToRange(x1, y1, x2, y2, Toggle);
    public void TurnOnRange(int x1, int y1, int x2, int y2) => ApplyToRange(x1, y1, x2, y2, TurnOn);
    public void TurnOffRange(int x1, int y1, int x2, int y2) => ApplyToRange(x1, y1, x2, y2, TurnOff);
    public int Get(int x, int y) => grid[x, y];

    private void Toggle(int x, int y) => Increase(x, y, 2);
    private void TurnOn(int x, int y) => Increase(x, y, 1);
    private void TurnOff(int x, int y) => Decrease(x, y, 1);
    private void ApplyToRange(int x1, int y1, int x2, int y2, ModifyLightFunction function)
    {
      for (int x = x1; x <= x2; x++)
        for (int y = y1; y <= y2; y++)
          function(x, y);
    }
    private void Increase(int x, int y, int v)
    {
      Set(x, y, Get(x, y) + v);
      brighteness += v;
    }
    private void Decrease(int x, int y, int v)
    {
      Set(x, y, Get(x, y) - v);
      if (brighteness > 0) brighteness -= v;
    }
    private void Set(int x, int y, int v) => grid[x, y] = v;
  }
  public static class Day6Part2
  {

    static (int, int) ParseRangeXY(string left)
    {
      int[] leftRange = left.Split(',').Select(int.Parse).ToArray();
      return (leftRange[0], leftRange[1]);
    }
    public static int Process(string input)
    {
      Lights2 lights = new Lights2(1000);

      string[] lines = input.Split('\n').Select(x => x.Trim()).ToArray();
      string text = string.Empty;
      foreach (string line in lines)
      {
        string[] split = line.Split(" through ").ToArray();
        string right = split[1];
        var (x2, y2) = ParseRangeXY(right);


        string left = split[0];
        if (left.Contains("turn on "))
        {
          left = left.Replace("turn on ", "");
          var (x1, y1) = ParseRangeXY(left);
          lights.TurnOnRange(x1, y1, x2, y2);
          string v = $"ON: ({x1}),({y1}) ({x2},{y2}) B: {lights.brighteness}";
          text += v + "\n";
          System.Console.WriteLine(v);
        }
        else if (left.Contains("toggle "))
        {
          left = left.Replace("toggle ", "");
          var (x1, y1) = ParseRangeXY(left);
          lights.ToggleRange(x1, y1, x2, y2);
          string v = $"TG: ({x1}),({y1}) ({x2},{y2}) B: {lights.brighteness}";
          text += v + "\n";
          System.Console.WriteLine(v);
        }
        else if (left.Contains("turn off "))
        {
          left = left.Replace("turn off ", "");
          var (x1, y1) = ParseRangeXY(left);
          lights.TurnOffRange(x1, y1, x2, y2);
          string v = $"OF: ({x1}),({y1}) ({x2},{y2}) B: {lights.brighteness}";
          text += v + "\n";
          System.Console.WriteLine(v);
        }
      }

      string path = "year-2015/day-6/output.txt";
      File.WriteAllText(path, text);

      return lights.Total();
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-6/input.txt");
      return Day6Part2.Process(input);
    }
  }
  public static class Day6Part2Test
  {
    public static int TestProcess1() => Day6Part2.Process("turn on 0,0 through 0,0");
    public static int TestProcess2() => Day6Part2.Process("toggle 0,0 through 999,999");
  }
}
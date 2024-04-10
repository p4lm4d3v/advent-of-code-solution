
using System.Security.Cryptography.X509Certificates;

namespace Solution.year_2015.day_6
{
  public class Lights1
  {
    private delegate void ModifyLightFunction(int x, int y);
    private bool[,] grid;
    public int on;

    public Lights1(int size)
    {
      grid = new bool[size, size];
      on = 0;
    }

    public void ToggleRange(int x1, int y1, int x2, int y2) => ApplyToRange(x1, y1, x2, y2, Toggle);
    public void TurnOnRange(int x1, int y1, int x2, int y2) => ApplyToRange(x1, y1, x2, y2, TurnOn);
    public void TurnOffRange(int x1, int y1, int x2, int y2) => ApplyToRange(x1, y1, x2, y2, TurnOff);
    public bool Get(int x, int y) => grid[x, y];

    private void Toggle(int x, int y)
    {
      bool before = Get(x, y);
      Set(x, y, !Get(x, y));
      bool after = Get(x, y);
      if (before == false && after == true) on++;
      else if (before == true && after == false) on--;
    }
    private void TurnOn(int x, int y)
    {
      bool before = Get(x, y);
      Set(x, y, true);
      if (!before) on++;
    }
    private void TurnOff(int x, int y)
    {
      bool before = Get(x, y);
      Set(x, y, false);
      if (before) on--;
    }
    private void ApplyToRange(int x1, int y1, int x2, int y2, ModifyLightFunction function)
    {
      for (int x = x1; x <= x2; x++)
        for (int y = y1; y <= y2; y++)
          function(x, y);
    }
    private void Set(int x, int y, bool v) => grid[x, y] = v;
  }
  public static class Day6Part1
  {
    public static int Process(string input)
    {
      Lights1 lights = new Lights1(1000);

      string[] lines = input.Split('\n').Select(x => x.Trim()).ToArray();
      foreach (string line in lines)
      {
        string[] split = line.Split(" through ").ToArray();
        int[] rightRange = split[1].Split(',').Select(int.Parse).ToArray();
        int x2 = rightRange[0];
        int y2 = rightRange[1];

        (int, int) Parse(string left)
        {
          int[] leftRange = left.Split(',').Select(int.Parse).ToArray();
          return (leftRange[0], leftRange[1]);
        }

        string left = split[0];
        if (left.Contains("turn on "))
        {
          left = left.Replace("turn on ", "");
          var (x1, y1) = Parse(left);
          lights.TurnOnRange(x1, y1, x2, y2);
        }
        else if (left.Contains("toggle "))
        {
          left = left.Replace("toggle ", "");
          var (x1, y1) = Parse(left);
          lights.ToggleRange(x1, y1, x2, y2);
        }
        else if (left.Contains("turn off "))
        {
          left = left.Replace("turn off ", "");
          var (x1, y1) = Parse(left);
          lights.TurnOffRange(x1, y1, x2, y2);
        }
      }

      return lights.on;
    }
    public static int Run()
    {
      string input = File.ReadAllText("year-2015/day-6/input.txt");
      return Day6Part1.Process(input);
    }
  }
  public static class Day6Part1Test
  {
    public static int TestProcess1() => Day6Part1.Process("turn on 0,0 through 999,999");
    public static int TestProcess2() => Day6Part1.Process("toggle 0,0 through 999,0");
    public static int TestProcess3() => Day6Part1.Process("turn off 499,499 through 500,500");
  }
}
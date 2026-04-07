using BenchmarkDotNet.Running;

namespace LocalizationChallenge.Benchmarks;

public class Program
{
    public static void Main()
    {
        BenchmarkRunner.Run<LocalizationBenchmarks>(new BenchmarkConfig());
    }
}
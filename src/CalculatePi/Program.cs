using System.Diagnostics;

public class Program
{
    public static void Main()
    {
        long totalIterations = 1_000_000_000;
        Stopwatch stopwatch = Stopwatch.StartNew();
        for (int i = 0; i < 10; i++)
        {
            stopwatch.Restart();
            long inCircle = PiEstimatorV0(totalIterations);
            // long inCircle = PiEstimatorV1(totalIterations);
            // long inCircle = PiEstimatorV2(totalIterations);
            // long inCircle = PiEstimatorV3(totalIterations);
            // long inCircle = PiEstimatorV4(totalIterations);
            // long inCircle = PiEstimatorV5(totalIterations);
            Console.WriteLine($"Estimated value of π: {4.0 * inCircle / totalIterations} in {stopwatch.ElapsedMilliseconds} ms");
        }
    }

    public static long PiEstimatorV5(long iterations)
    {
        int coreCount = Environment.ProcessorCount;
        long chunkSize = iterations / coreCount;
        long inCircle = 0;

        var tasks = Enumerable.Range(0, coreCount).Select(id => Task.Run(() =>
        {
            var rng = new Random();
            long localCount = 0;
            long start = id * chunkSize;
            long end = id == coreCount - 1 ? iterations : start + chunkSize;

            for (long i = start; i < end; i++)
            {
                double x = rng.NextDouble();
                double y = rng.NextDouble();
                if (x * x + y * y <= 1.0)
                    localCount++;
            }

            Interlocked.Add(ref inCircle, localCount);
        })).ToArray();

        Task.WaitAll(tasks);
        return inCircle;
    }

    public static long PiEstimatorV4(long iterations)
    {
        long inCircle = 0;
        using var tlRandom = new ThreadLocal<Random>(() => new Random());

        Parallel.For(
            0L,
            iterations,
            () => 0L,
            (_, _, localCount) =>
            {
                var rng = tlRandom.Value!;
                double x = rng.NextDouble();
                double y = rng.NextDouble();
                return x * x + y * y <= 1.0 ? localCount + 1 : localCount;
            },
            localCount => Interlocked.Add(ref inCircle, localCount)
        );

        return inCircle;
    }

    public static long PiEstimatorV3(long iterations)
    {
        long inCircle = 0;
        var random = new Random();

        for (int i = 0; i < iterations; i++)
        {
            double x = random.NextDouble();
            double y = random.NextDouble();

            if (x * x + y * y <= 1.0)
            {
                Interlocked.Increment(ref inCircle);
            }
        }
        return inCircle;
    }

    public static long PiEstimatorV2(long iterations)
    {
        long inCircle = 0;
        var random = new Random();

        Parallel.For(0, iterations, i =>
        {
            double x = random.NextDouble();
            double y = random.NextDouble();

            if (x * x + y * y <= 1.0)
            {
                Interlocked.Increment(ref inCircle);
            }
        }
        );
        return inCircle;
    }

    public static long PiEstimatorV1(long iterations)
    {
        long inCircle = 0;
        var random = new Random();

        for (int i = 0; i < iterations; i++)
        {
            double x = random.NextDouble();
            double y = random.NextDouble();
            if (Math.Pow(x, 2) + Math.Pow(y, 2) <= 1.0)
            {
                inCircle++;
            }
        }
        return inCircle;
    }

    public static long PiEstimatorV0(long iterations)
    {
        long inCircle = 0;
        var random = new Random();

        for (int i = 0; i < iterations; i++)
        {
            double x = random.NextDouble();
            double y = random.NextDouble();
            if (x * x + y * y <= 1.0)
            {
                inCircle++;
            }
        }
        return inCircle;
    }
}
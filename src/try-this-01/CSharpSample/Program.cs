namespace CSharpSample;

// CASE02: Interlocked sınıfı kullanımı ile veri tutarlılığının sağlanması

using System.Threading;
public class Program
{
    static double CalculationResult = 0;

    static void Main(string[] args)
    {
        Thread threadA = new(PerformCalculationsA);
        Thread threadB = new(PerformCalculationsB);

        threadA.Start();
        threadB.Start();

        threadA.Join();
        threadB.Join();

        Console.WriteLine($"Final Calculation Result: {CalculationResult}");
    }

    static void PerformCalculationsA()
    {
        for (int i = 0; i < 100; i++)
        {
            double partialResult = Math.Sqrt(i);
            double initialValue, computedValue;
            do
            {
                initialValue = CalculationResult;
                computedValue = initialValue + partialResult;
            } while (Interlocked.CompareExchange(ref CalculationResult, computedValue, initialValue) != initialValue);
            Thread.Sleep(50);
        }
    }

    static void PerformCalculationsB()
    {
        for (int i = 0; i < 100; i++)
        {
            double partialResult = Math.Log(i + 1);
            double initialValue, computedValue;
            do
            {
                initialValue = CalculationResult;
                computedValue = initialValue + partialResult;
            } while (Interlocked.CompareExchange(ref CalculationResult, computedValue, initialValue) != initialValue);
            Thread.Sleep(50);
        }
    }
}

// // CASE01: CASE00'daki problemin çözümü için kilit mekanizması kullanımı
// // En basit kilit mekanizması olarak lock kullanılıyoruz.
// using System.Threading;
// public class Program
// {
//     static double CalculationResult = 0;
//     static readonly Lock calculationLock = new();

//     static void Main(string[] args)
//     {
//         Thread threadA = new(PerformCalculationsA);
//         Thread threadB = new(PerformCalculationsB);

//         threadA.Start();
//         threadB.Start();

//         threadA.Join();
//         threadB.Join();

//         Console.WriteLine($"Final Calculation Result: {CalculationResult}");
//     }

//     static void PerformCalculationsA()
//     {
//         for (int i = 0; i < 100; i++)
//         {
//             double partialResult = Math.Sqrt(i);
//             lock (calculationLock)
//             {
//                 CalculationResult += partialResult;
//             }
//             Thread.Sleep(50);
//         }
//     }

//     static void PerformCalculationsB()
//     {
//         for (int i = 0; i < 100; i++)
//         {
//             double partialResult = Math.Log(i + 1);
//             lock (calculationLock)
//             {
//                 CalculationResult += partialResult;
//             }
//             Thread.Sleep(50);
//         }
//     }
// }

// // CASE00: İki farklı thread'in aynı paylaşılan değişken üzerinde değişiklik yapması durumu
// public class Program
// {
//     static double CalculationResult = 0;

//     static void Main(string[] args)
//     {
//         Thread threadA = new(PerformCalculationsA);
//         Thread threadB = new(PerformCalculationsB);

//         threadA.Start();
//         threadB.Start();

//         threadA.Join();
//         threadB.Join();

//         Console.WriteLine($"Final Calculation Result: {CalculationResult}");
//     }

//     static void PerformCalculationsA()
//     {
//         for (int i = 0; i < 100; i++)
//         {
//             CalculationResult += Math.Sqrt(i);
//             Thread.Sleep(50);
//         }
//     }

//     static void PerformCalculationsB()
//     {
//         for (int i = 0; i < 100; i++)
//         {
//             CalculationResult += Math.Log(i + 1);
//             Thread.Sleep(50);
//         }
//     }
// }
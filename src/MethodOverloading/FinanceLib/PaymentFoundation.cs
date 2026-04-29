using System.Runtime.InteropServices;

namespace FinanceLib;

public class PaymentFoundation
{
    public static void ProcessPayment(decimal amount)
    {
        Console.WriteLine($"Processing payment of {amount:C}");
    }

    public static void ProcessPayment(int bonus)
    {
        Console.WriteLine($"Processing payment of {bonus} bonus points");
    }

    // Publish error. The method signatures are the same, so we need to give them different entry points.

    // [UnmanagedCallersOnly(EntryPoint = "ProcessPayment")]
    // public static void ExportProcessPayment(double amount) => ProcessPayment((decimal)amount);

    // [UnmanagedCallersOnly(EntryPoint = "ProcessPayment")]
    // public static void ExportProcessPayment(int bonus) => ProcessPayment(bonus);

    [UnmanagedCallersOnly(EntryPoint = "ProcessPayment_WithAmount")]
    public static void ExportProcessPayment(double amount) => ProcessPayment((decimal)amount);

    [UnmanagedCallersOnly(EntryPoint = "ProcessPayment_WithBonus")]
    public static void ExportProcessPayment(int bonus) => ProcessPayment(bonus);
}

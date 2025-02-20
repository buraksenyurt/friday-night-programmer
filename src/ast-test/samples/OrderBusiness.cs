namespace Fnp.Works;

public class OrderBusiness : BusinessBase
{
    public int OrderCount { get; set; }
    private decimal TotalRevenue { get; set; }

    public bool ApproveOrder(int orderId)
    {
        return true;
    }

    private void LogOrder(int orderId)
    {
        Console.WriteLine($"Order {orderId} has been logged.");
    }

    protected decimal CalculateDiscount(decimal amount)
    {
        return amount * 0.9m;
    }
}

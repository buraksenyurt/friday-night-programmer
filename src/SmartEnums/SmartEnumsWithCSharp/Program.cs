using System;

namespace SmartEnumsWithCSharp;

public enum OrderFormStatus
{
    Draft,
    Canceled,
    Completed,
    Processing
}

public class OrderFormTenantStatus // : ValueObject
{
    public Guid Id { get; }
    public Guid TenantId { get; }
    public string Name { get; }
    public OrderFormStatus CoreStatus { get; }

    private OrderFormTenantStatus(Guid id, Guid tenantId, string name, OrderFormStatus coreStatus)
    {
        if (string.IsNullOrEmpty(name))
            throw new ArgumentException("Name cannot be null or empty.", nameof(name));

        Id = id;
        TenantId = tenantId;
        Name = name;
        CoreStatus = coreStatus;
    }

    public static readonly OrderFormTenantStatus Draft = new(Guid.NewGuid(), Guid.Empty, "Draft", OrderFormStatus.Draft);
    public static readonly OrderFormTenantStatus Processing = new(Guid.NewGuid(), Guid.Empty, "Processing", OrderFormStatus.Processing);
    public static readonly OrderFormTenantStatus Completed = new(Guid.NewGuid(), Guid.Empty, "Completed", OrderFormStatus.Completed);
    public static readonly OrderFormTenantStatus Canceled = new(Guid.NewGuid(), Guid.Empty, "Canceled", OrderFormStatus.Canceled);

    public static OrderFormTenantStatus Create(Guid id, Guid tenantId, string name, OrderFormStatus mappedCoreStatus)
    {
        return new OrderFormTenantStatus(id, tenantId, name, mappedCoreStatus);
    }
}

public class OrderForm
{
    public Guid Id { get; private set; }
    public Guid TenantId { get; private set; }
    public OrderFormTenantStatus Status { get; private set; }

    public OrderForm(Guid id, Guid tenantId, OrderFormTenantStatus initialStatus)
    {
        Id = id;
        TenantId = tenantId;
        
        if (initialStatus.CoreStatus != OrderFormStatus.Draft)
            throw new ArgumentException("Initial status must be Draft.", nameof(initialStatus));

        Status = initialStatus;
    }

    public void UpdateStatus(OrderFormTenantStatus newStatus)
    {
        if (newStatus.TenantId != Guid.Empty && newStatus.TenantId != TenantId)
            throw new InvalidOperationException("Cannot change status to a status from a different tenant.");

        OrderFormStatus currentCoreStatus = Status.CoreStatus;
        OrderFormStatus newCoreStatus = newStatus.CoreStatus;

        if (currentCoreStatus == OrderFormStatus.Draft && newCoreStatus != OrderFormStatus.Processing)
            throw new InvalidOperationException("Draft status can only transition to Processing.");

        if (currentCoreStatus == OrderFormStatus.Processing && newCoreStatus == OrderFormStatus.Draft)
            throw new InvalidOperationException("Processing status cannot transition back to Draft.");

        Status = newStatus;
    }
}

public class Program
{
    public static void Main()
    {
        Guid myTenantId = Guid.NewGuid();

        var waitingForPartsStatus = OrderFormTenantStatus.Create(Guid.NewGuid(), myTenantId, "Waiting for Parts", OrderFormStatus.Processing);
        var assemblyLineStatus = OrderFormTenantStatus.Create(Guid.NewGuid(), myTenantId, "Assembly Line", OrderFormStatus.Processing);
        var inTailorStatus = OrderFormTenantStatus.Create(Guid.NewGuid(), myTenantId, "In Tailor", OrderFormStatus.Processing);

        var order = new OrderForm(Guid.NewGuid(), myTenantId, OrderFormTenantStatus.Draft);
        Console.WriteLine($"Initial Order Status: {order.Status.Name}");
        
        order.UpdateStatus(waitingForPartsStatus);
        Console.WriteLine($"Updated Order Status: {order.Status.Name}");
        
        order.UpdateStatus(assemblyLineStatus);
        Console.WriteLine($"Updated Order Status: {order.Status.Name}");
        
        order.UpdateStatus(inTailorStatus);
        Console.WriteLine($"Updated Order Status: {order.Status.Name}");
        
        order.UpdateStatus(OrderFormTenantStatus.Canceled);
        Console.WriteLine($"Updated Order Status: {order.Status.Name}");
    }
}
using Microsoft.EntityFrameworkCore;
using RequestFormApi.Model;

namespace RequestFormApi.Db;

public class DealerDbContext
    : DbContext
{
    public DbSet<ServiceRequestForm> ServiceRequestForms { get; set; }
    public DbSet<OutboxMessage> OutboxMessages { get; set; }

    public DealerDbContext(DbContextOptions<DealerDbContext> options)
        : base(options)
    {
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<OutboxMessage>()
            .Property(o => o.Id)
            .ValueGeneratedOnAdd();
    }
}

using Microsoft.EntityFrameworkCore;
using RequestFormApi.Model;

namespace RequestFormApi.Db;

public class DealerDbContext(DbContextOptions<DealerDbContext> options)
        : DbContext(options)
{
    public DbSet<ServiceRequestForm> ServiceRequestForms { get; set; }
    public DbSet<OutboxMessage> OutboxMessages { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<OutboxMessage>()
            .Property(o => o.Id)
            .ValueGeneratedOnAdd();
    }
}

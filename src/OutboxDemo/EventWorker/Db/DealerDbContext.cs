using EventWorker.Model;
using Microsoft.EntityFrameworkCore;

namespace EventWorker.Db;

public class DealerDbContext
    : DbContext
{
    public DbSet<OutboxMessage> OutboxMessages => Set<OutboxMessage>();

    public DealerDbContext(DbContextOptions<DealerDbContext> options)
        : base(options)
    {
    }
}

using EventWorker.Model;
using Microsoft.EntityFrameworkCore;

namespace EventWorker.Db;

public class DealerDbContext(DbContextOptions<DealerDbContext> options)
        : DbContext(options)
{
    public DbSet<OutboxMessage> OutboxMessages => Set<OutboxMessage>();
}

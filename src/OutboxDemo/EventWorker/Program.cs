using EventWorker;
using EventWorker.Db;
using Microsoft.EntityFrameworkCore;

var builder = Host.CreateApplicationBuilder(args);
builder.Services.AddDbContext<DealerDbContext>(
    options => options.UseNpgsql("Host=localhost;Database=dealer;Username=johndoe;Password=somew0rds")
    );
builder.Services.AddHostedService<Worker>();

var host = builder.Build();
host.Run();

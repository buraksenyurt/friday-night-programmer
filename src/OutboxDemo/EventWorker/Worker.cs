using Microsoft.EntityFrameworkCore;
using RabbitMQ.Client;
using System.Text;
using EventWorker.Db;

namespace EventWorker;

//TODO@buraksenyurt Klasik kullanım yerine Quartz kullanımı da denenmeli
public class Worker : BackgroundService
{
    private readonly ILogger<Worker> _logger;
    private readonly IServiceProvider _serviceProvider;

    public Worker(ILogger<Worker> logger, IServiceProvider serviceProvider)
    {
        _logger = logger;
        _serviceProvider = serviceProvider;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                using var scope = _serviceProvider.CreateScope();
                var dbContext = scope.ServiceProvider.GetRequiredService<DealerDbContext>();

                var pendingMessages = await dbContext.OutboxMessages
                                              .Where(m => !m.IsSent)
                                              .ToListAsync(stoppingToken);

                if (pendingMessages.Count != 0)
                {
                    // PoC amaçlı. Gerçek hayatta böyle yapmayın :D
                    var factory = new ConnectionFactory
                    {
                        HostName = "rabbitmq",
                        UserName = "guest",
                        Password = "guest"
                    };

                    using var connection = await factory.CreateConnectionAsync(stoppingToken);
                    using var channel = await connection.CreateChannelAsync(cancellationToken: stoppingToken);

                    await channel.QueueDeclareAsync(queue: "outbox_events", durable: false, exclusive: false, autoDelete: false, arguments: null, cancellationToken: stoppingToken);

                    foreach (var message in pendingMessages)
                    {
                        var body = Encoding.UTF8.GetBytes(message.Payload);
                        await channel.BasicPublishAsync(exchange: "", routingKey: "outbox_events", body: body, cancellationToken: stoppingToken);
                        message.IsSent = true;
                        message.SendDate = DateTime.UtcNow;

                        _logger.LogInformation("Mesaj gönderildi: {Id}", message.Id);
                    }

                    await dbContext.SaveChangesAsync(stoppingToken);
                }
                else
                {
                    _logger.LogInformation("Gönderim için bekleyen mesaj yok.");
                }
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Worker sırasında hata oluştu.");
            }

            await Task.Delay(TimeSpan.FromSeconds(10), stoppingToken);
        }
    }
}

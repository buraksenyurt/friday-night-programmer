using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using Npgsql;
using StackExchange.Redis;

namespace LocalizationChallenge.Infrastructure;

public class LocalizationCacheSenseService(
    NpgsqlDataSource dataSource,
    IConnectionMultiplexer redis,
    MemoryCacheLocalizationProvider memCacheProvider,
    ILogger<LocalizationCacheSenseService> logger) : BackgroundService
{
    protected override async Task ExecuteAsync(CancellationToken cancellationToken)
    {
        // Bir iptal isteği gelene kadar, PostgreSQL veritabanına bağlanarak "loc_changed" kanalını dinliyoruz.
        while (!cancellationToken.IsCancellationRequested)
        {
            try
            {
                await using var conn = await dataSource.OpenConnectionAsync(cancellationToken);

                // Notification event'ine abone olarak, veritabanında bir değişiklik olduğunda tetiklenecek kod bloğu.
                conn.Notification += async (_, args) =>
                {
                    // Eğer bir değişiklik varsa (update,delete, insert) event metodun içerisine düşmemiz lazım
                    logger.LogInformation("Localization changed: {Payload}", args.Payload);
                    // postgresql tarafından gönderilen payload'u "culture:resourceKey" formatında bekliyoruz.
                    // Bu bilgiyi kullanarak, ilgili cache kaydını Redis'ten siliyoruz.
                    var parts = args.Payload.Split(':', 2);
                    if (parts.Length == 2)
                    {
                        var culture = parts[0];
                        var resourceKey = parts[1];
                        var db = redis.GetDatabase();
                        await db.HashDeleteAsync($"loc:{culture}", resourceKey);
                    }
                    // Ayrıca bellekte duran cache'i de güncellemek adına
                    // MemoryCacheLocalizationProvider'ın StartAsync metodunu çağırıyoruz
                    // Dolayısıyla debug ederken oraya da gidebiliyor olmamız lazım
                    await memCacheProvider.StartAsync(CancellationToken.None);
                };

                // PostgreSQL'de LISTEN komutunu kullanarak loc_changed kanalını dinlememiz gerekiyor.
                // Zira oradaki trigger içerisinden tetiklenen fonksiyon bu isimle bir yayın yapmakta
                await using var cmd = conn.CreateCommand();
                cmd.CommandText = "LISTEN loc_changed";
                await cmd.ExecuteNonQueryAsync(cancellationToken);

                logger.LogInformation("Listening for localization changes on PostgreSQL channel loc_changed");

                while (!cancellationToken.IsCancellationRequested)
                    await conn.WaitAsync(cancellationToken);
            }
            catch (OperationCanceledException)
            {
                break;
            }
            catch (Exception ex)
            {
                logger.LogWarning(ex, "Localization invalidation connection lost. Reconnecting in 5s...");
                await Task.Delay(TimeSpan.FromSeconds(5), cancellationToken);
            }
        }
    }
}

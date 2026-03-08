namespace SSEApi;

// Bu bileşenimiz, oyuncu skor bilgilerini belirli periyotlarda güncelleyen
// bir arka plan hizmeti olarak düşünülebilir. Sadece skor değişiklilerini
// anlık simüle etmek için kullanılıyor ve dikkat edileceği üzere gamerService'in
// UpdateStats() metodunu çağırarak bu işlemi yapıyor.
public class GamerServiceWorker(GamerService gamerService, ILogger<GamerServiceWorker> logger)
    : BackgroundService
{
    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        logger.LogInformation("GamerServiceWorker başlatıldı.");

        while (!stoppingToken.IsCancellationRequested)
        {
            gamerService.UpdateStats();
            await Task.Delay(TimeSpan.FromSeconds(5), stoppingToken);
        }

        logger.LogInformation("GamerServiceWorker durduruldu.");
    }
}
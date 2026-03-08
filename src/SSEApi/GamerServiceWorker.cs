namespace SSEApi;

// Bu bileşenimiz, oyuncu skor bilgilerini belirli periyotlarda güncelleyen
// bir arka plan hizmeti olarak düşünülebilir. Sadece skor değişiklilerini
// anlık simüle etmek için kullanılıyor ve dikkat edileceği üzere gamerService'in
// UpdateStats() metodunu çağırarak bu işlemi yapıyor.
public class GamerServiceWorker(GamerService gamerService) 
    : BackgroundService
{
    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        // Bir iptal sinyali gelene kadar çalışmaya devam eden döngümüz.
        while (!stoppingToken.IsCancellationRequested)
        {
            gamerService.UpdateStats();
            await Task.Delay(TimeSpan.FromSeconds(5), stoppingToken);
        }
    }
}
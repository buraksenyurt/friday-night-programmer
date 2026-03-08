using System.Runtime.CompilerServices;
using System.Threading.Channels;

namespace SSEApi;

// Asıl işin yapıldığı servis bileşenimiz.
public class GamerService
{
    private readonly Random random = new();
    private readonly List<Gamer> gamers;
    // Abone olan istemcilere güncellemeleri iletmek için kullandığımız kanal listesi 
    private readonly List<Channel<List<Gamer>>> _subscribers = [];
    // Eş zamanlı erişimlerde oluşabilecek sorunları önlemek için basit bir thread kilit mekanizması kullanıyoruz.
    // Bu nedenle bir Lock nesnemiz var.
    private readonly Lock _lock = new();

    public GamerService()
    {
        // Bunlar kobay oyuncularımız :D
        gamers =
        [
            new Gamer { Name = "Alicya", Score = 100 },
            new Gamer { Name = "Bobby", Score = 150 },
            new Gamer { Name = "Charles", Score = 120 },
            new Gamer { Name = "Dayana", Score = 130 },
            new Gamer { Name = "Evie", Score = 110 },
        ];
    }

    /*
        Belkide en kritik metodumuz burası olabilir.
        IAsyncEnumerable<List<Gamer>> türünden sonuç döndüren metotlar server event olarak kullanılabiliyorlar.
        Bu metodumuzun görevi, abone olan istemcilere güncellemeleri iletmek. 
        Abone olan her istemci için bir kanal oluşturulup _subscribers listesine ekleniyor. 
        İstemci bağlantısı kesildiğinde ise ilgili kanal listeden çıkarılıyor.
    */
    public async IAsyncEnumerable<List<Gamer>> SubscribeAsync(
        [EnumeratorCancellation] CancellationToken cancellationToken)
    {
        // Abone olan istemci için yeni bir kanal oluşturur.
        var channel = Channel.CreateUnbounded<List<Gamer>>();

        // thread safe olması için kilit açılır
        lock (_lock)
        {
            // abone eklenir ve mevcut oyuncu listesi iletilir
            _subscribers.Add(channel);
            channel.Writer.TryWrite([.. gamers]);
        }

        try
        {
            // Abone olan istemci için oluşturulan kanaldan güncellemeleri okur ve iletir.
            await foreach (var update in channel.Reader.ReadAllAsync(cancellationToken))
            {
                yield return update;
            }
        }
        finally
        {
            lock (_lock) _subscribers.Remove(channel);
            channel.Writer.Complete();
        }
    }

    // Bu metot, background service tarafından belirli periyotlarda çağırılır.
    // Kendince oyuncu skorlarını rastgele günceller.
    // Buna bağlı olarak gamers listesi de yenileni ve güncellenmiş liste tüm abonelere iletilir.
    public void UpdateStats()
    {
        foreach (var gamer in gamers)
        {
            gamer.Score += random.Next(-10, 20);
        }

        var snapshot = gamers.ToList();

        // thread-safe bir kod bloğu açılır
        lock (_lock)
        {
            // tüm abonelere güncellenmiş skor listesi iletilir
            foreach (var subscriber in _subscribers)
            {
                subscriber.Writer.TryWrite(snapshot);
            }
        }
    }
}
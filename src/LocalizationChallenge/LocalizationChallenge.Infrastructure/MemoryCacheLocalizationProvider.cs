using LocalizationChallenge.Core;
using Microsoft.Extensions.Hosting;
using Npgsql;
using System.Collections.Frozen;

namespace LocalizationChallenge.Infrastructure;

public sealed class MemoryCacheLocalizationProvider(NpgsqlDataSource dataSource)
    : ILocalizationProvider, IHostedService
{
    // Değişmeyen veri yapılarında, thread-safe olan ve hızlı erişim sağlayan FrozenDictionary bileşenini kullanarak önbellek oluşturuyoruz.
    // volatile bildirimi ile cache değişkenine yapılan atamaların tüm thread'ler tarafından görünür olması sağlanır.
    // Böylece StartAsync metodunda cache güncellendiğinde, diğer thread'ler de bu güncellemeyi görebilir.
    private volatile FrozenDictionary<string, FrozenDictionary<string, string>> cache = FrozenDictionary<string, FrozenDictionary<string, string>>.Empty;
    public string ProviderName => "InMemoryCache";

    public ValueTask<string> GetLocalizedStringAsync(string key, string culture, CancellationToken cancellationToken = default)
    {
        if (cache.TryGetValue(culture, out var dict) && dict.TryGetValue(key, out var val))
            return ValueTask.FromResult<string?>(val);

        return ValueTask.FromResult<string?>(null);
    }

    // Bileşenimizi başlatırken, veritabanından tüm lokalizasyon verilerini çekip, hızlı erişim için önbelleğe yüklememiz gerekiyor.
    // Bunun için IHostedService arayüzü implementasyonunu kullandık.
    // Api tarafındaki DI container'ına bu bileşeni singleton olarak eklerken, aynı zamanda IHostedService olarak da kaydedeceğiz.
    // Böylece uygulama başladığında StartAsync metodu tetiklenecek ve önbellek doldurulacak.
    public async Task StartAsync(CancellationToken cancellationToken)
    {
        await using var cmd = dataSource.CreateCommand(
            "SELECT culture, resource_key, value FROM localizations");
        await using var reader = await cmd.ExecuteReaderAsync(cancellationToken);

        var staging = new Dictionary<string, Dictionary<string, string>>(StringComparer.OrdinalIgnoreCase);

        while (await reader.ReadAsync(cancellationToken))
        {
            var culture = reader.GetString(0);
            var resourceKey = reader.GetString(1);
            var value = reader.GetString(2);

            if (!staging.TryGetValue(culture, out var dict))
            {
                dict = new Dictionary<string, string>(StringComparer.Ordinal);
                staging[culture] = dict;
            }
            dict[resourceKey] = value;
        }

        cache = staging.ToFrozenDictionary(
            kvp => kvp.Key,
            kvp => kvp.Value.ToFrozenDictionary(StringComparer.Ordinal),
            StringComparer.OrdinalIgnoreCase);
    }

    public Task StopAsync(CancellationToken cancellationToken) => Task.CompletedTask;
}

using LocalizationChallenge.Core;
using StackExchange.Redis;

namespace LocalizationChallenge.Infrastructure;

public sealed class HybridLocalizationProvider(
    MemoryCacheLocalizationProvider level1,
    RedisLocalizationProvider level2,
    PostgresLocalizationProvider level3,
    IConnectionMultiplexer redis) : ILocalizationProvider
{
    private readonly IDatabase redisDb = redis.GetDatabase();

    public string ProviderName => "Hybrid";

    public async ValueTask<string> GetLocalizedStringAsync(string key, string culture, CancellationToken cancellationToken = default)
    {
        var value = await level1.GetLocalizedStringAsync(culture, key, cancellationToken);
        if (value is not null) return value;

        value = await level2.GetLocalizedStringAsync(culture, key, cancellationToken);
        if (value is not null) return value;

        value = await level3.GetLocalizedStringAsync(culture, key, cancellationToken);
        if (value is not null)
        {
            _ = redisDb.HashSetAsync($"loc:{culture}", key, value);
        }

        return value;
    }
}

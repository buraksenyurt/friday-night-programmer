using LocalizationChallenge.Core;
using StackExchange.Redis;

namespace LocalizationChallenge.Infrastructure;

public sealed class RedisLocalizationProvider(IConnectionMultiplexer connectionMultiplexer)
    : ILocalizationProvider
{
    private readonly IDatabase database = connectionMultiplexer.GetDatabase();
    public string ProviderName => "Redis";

    public async ValueTask<string> GetLocalizedStringAsync(string key, string culture, CancellationToken cancellationToken = default)
    {
        var value = await database.HashGetAsync($"loc::{culture}", key);
        return value.HasValue ? value.ToString() : string.Empty;
    }
}

using LocalizationChallenge.Core;
using Npgsql;

namespace LocalizationChallenge.Infrastructure;

public sealed class PostgresLocalizationProvider(NpgsqlDataSource dataSource)
    : ILocalizationProvider
{
    public string ProviderName => "Postgres";

    public async ValueTask<string> GetLocalizedStringAsync(string key, string culture, CancellationToken cancellationToken = default)
    {
        await using var command = dataSource.CreateCommand(
            "SELECT value FROM localizations WHERE key = @key AND culture = @culture LIMIT 1"
        );
        command.Parameters.AddWithValue("key", key);
        command.Parameters.AddWithValue("culture", culture);

        var result = await command.ExecuteScalarAsync(cancellationToken);
        return result as string ?? string.Empty;
    }
}

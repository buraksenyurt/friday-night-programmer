using LocalizationChallenge.Core;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Npgsql;
using StackExchange.Redis;

namespace LocalizationChallenge.Infrastructure;

public static class DependencyInjection
{
    public static IServiceCollection AddLocalizationProviders(this IServiceCollection services, IConfiguration configuration)
    {
        var pgDataSource = NpgsqlDataSource.Create(configuration.GetConnectionString("Postgres"));
        services.AddSingleton(pgDataSource);

        var redis = ConnectionMultiplexer.Connect(configuration.GetConnectionString("Redis"));
        services.AddSingleton<IConnectionMultiplexer>(redis);

        services.AddSingleton<MemoryCacheLocalizationProvider>();
        services.AddSingleton<RedisLocalizationProvider>();
        services.AddSingleton<PostgresLocalizationProvider>();

        services.AddSingleton<ILocalizationProvider>(sp => sp.GetRequiredService<MemoryCacheLocalizationProvider>());
        services.AddSingleton<ILocalizationProvider>(sp => sp.GetRequiredService<RedisLocalizationProvider>());
        services.AddSingleton<ILocalizationProvider>(sp => sp.GetRequiredService<PostgresLocalizationProvider>());
        services.AddSingleton<ILocalizationProvider, HybridLocalizationProvider>();

        services.AddHostedService(sp => sp.GetRequiredService<MemoryCacheLocalizationProvider>());
        services.AddHostedService<LocalizationCacheSenseService>();

        return services;
    }
}

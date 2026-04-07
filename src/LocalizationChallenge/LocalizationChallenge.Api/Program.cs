using LocalizationChallenge.Core;
using LocalizationChallenge.Infrastructure;
using Microsoft.AspNetCore.Mvc;
using System.Diagnostics;

var builder = WebApplication.CreateBuilder(args);

// Postgres ve Redis için connection string bilgileri alınıyor.
var postgresConnStr = builder.Configuration.GetConnectionString("Postgres") ?? throw new InvalidOperationException("Postgres connection string is not configured.");
var redisConnStr = builder.Configuration.GetConnectionString("Redis") ?? throw new InvalidOperationException("Redis connection string is not configured.");

// Localization provider'lar DI container'a ekleniyor.
builder.Services.AddLocalizationProviders(builder.Configuration);

builder.Services.AddOpenApi();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}

app.UseHttpsRedirection();

// Adettendir "api ayakta mı?" kontrolü
app.MapGet("api/health", () => Results.Ok("Healthy"));

// Belli bir provider, culture ve key için lokalize edilmiş string değeri döndüren endpoint.
app.MapGet("api/localization/{provider}/{culture}/{key}", async (
    string provider,
    string culture,
    string key,
    [FromServices] IEnumerable<ILocalizationProvider> providers,
    CancellationToken cancellationToken) =>
{
    // [FromServices] ile tüm ILocalizationProvider implementasyonlarını alıyoruz ve istenen provider adına sahip olanı buluyoruz.
    var target = providers.FirstOrDefault(p => p.ProviderName.Equals(provider, StringComparison.OrdinalIgnoreCase));
    if (target is null) return Results.NotFound(new { error = $"Provider '{provider}' not found." });

    // Performans ölçümü için Stopwatch kullanarak, lokalize string alma işleminin ne kadar sürdüğünü hesaplıyoruz.
    var timer = Stopwatch.GetTimestamp();
    var value = await target.GetLocalizedStringAsync(key, culture, cancellationToken);
    var elapsed = Stopwatch.GetElapsedTime(timer);

    // Ölçüm sonuçlarını dönüyoruz
    return Results.Ok(new
    {
        Provider = target.ProviderName,
        Culture = culture,
        Key = key,
        Value = value,
        ElapsedMicroseconds = elapsed.TotalMicroseconds
    });
}).WithDescription("Get localized string by provider, culture, and key.");


// Tüm provider'lar için belli bir culture ve key'e karşılık gelen string değerleri döndüren endpoint.
// Bunu tüm provider'ların aynı key için aynı değeri döndürüp döndürmediğini kontrol etmek ve performans karşılaştırması yapmak için kullanabiliriz.
app.MapGet("api/benchmark/{culture}/{key}", async (
    string culture,
    string key,
    [FromServices] IEnumerable<ILocalizationProvider> providers,
    CancellationToken cancellationToken) =>
{
    var results = new List<BenchmarkResult>();

    // DI Container'a kayıtlı tüm provider'lar için, verilen culture ve key'e karşılık gelen lokalize string değerini alıp,
    // performans ölçümü yaparak sonuçları topluyoruz.
    foreach (var provider in providers)
    {
        var timer = Stopwatch.GetTimestamp();
        var value = await provider.GetLocalizedStringAsync(key, culture, cancellationToken);
        var elapsed = Stopwatch.GetElapsedTime(timer);

        results.Add(new BenchmarkResult(
            provider.ProviderName,
            value,
            elapsed.TotalMicroseconds,
            CacheHit: value is not null));
    }

    return Results.Ok(new
    {
        Culture = culture,
        Key = key,
        MeasuredAt = DateTime.UtcNow,
        Results = results.OrderBy(r => r.ElapsedMicroseconds)
    });
}).WithDescription("Benchmark all providers for a given culture and key.");

await app.RunAsync();
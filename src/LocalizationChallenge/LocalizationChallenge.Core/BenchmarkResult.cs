namespace LocalizationChallenge.Core;

public sealed record BenchmarkResult(
    string ProviderName,
    string? Value,
    double ElapsedMicroseconds,
    bool CacheHit
);

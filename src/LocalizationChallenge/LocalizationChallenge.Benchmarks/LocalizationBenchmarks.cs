using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Order;
using LocalizationChallenge.Infrastructure;
using Npgsql;
using StackExchange.Redis;

namespace LocalizationChallenge.Benchmarks;

[Config(typeof(BenchmarkConfig))]
[MemoryDiagnoser]
[Orderer(SummaryOrderPolicy.FastestToSlowest)]
public class LocalizationBenchmarks
{
    private PostgresLocalizationProvider _postgres = null!;
    private RedisLocalizationProvider _redis = null!;
    private MemoryCacheLocalizationProvider _memory = null!;
    private HybridLocalizationProvider _hybrid = null!;

    /*
        Testimi sadece tek bir metin için değil 6 farklı senaryo için koşulacak.
        3 culture * 2 key olarak düşünebiliriz.
        Yani çalışma zamanında benchmark buradaki parametrelere göre olası tüm kombinasyonları işletecektir.
    */
    [Params("tr-TR", "en-US", "de-DE")]
    public string Culture { get; set; } = "tr-TR";

    [Params("welcome_message", "button_save")]
    public string Key { get; set; } = "welcome_message";

    /*
        Setup metodu sadece bir kez çalışır. Sonuçta benchmark ölçümlerinde
        sistem ayağa kaklarken veritabanı bağlantısının oluşturulması, redis'e bağlanılması 
        ve provider nesnelerinin bunları kullanarak oluşturulması gibi işlemler de dahil olmak 
        üzere tüm hazırlıkların yapılması gerekir.
        Bunlar ölçümlerimizi etkilememli zira ölçmek istediğimi konu bu hazırlık safhası değil.
    */
    [GlobalSetup]
    public async Task Setup()
    {
        const string postgresConn = "Host=localhost;Port=5435;Database=postgres;Username=johndoe;Password=somew0rds";
        const string redisConn = "localhost:6379,abortConnect=false";

        var dataSource = NpgsqlDataSource.Create(postgresConn);
        var redisDb = await ConnectionMultiplexer.ConnectAsync(redisConn);

        _postgres = new PostgresLocalizationProvider(dataSource);
        _redis = new RedisLocalizationProvider(redisDb);
        _memory = new MemoryCacheLocalizationProvider(dataSource);
        _hybrid = new HybridLocalizationProvider(_memory, _redis, _postgres, redisDb);

        await (_memory).StartAsync(CancellationToken.None);
    }

    /*
        Burası yarışmacıların tanımlandığı kısımdır.
        Buradaki metotların her biri 10 tur boyunca 112şer kez çağrılacak. 
    */
    [Benchmark(Baseline = true, Description = "PostgreSQL (no cache)")]
    public ValueTask<string?> PostgreSQL() => _postgres.GetLocalizedStringAsync(Culture, Key);

    [Benchmark(Description = "Redis (single key)")]
    public ValueTask<string?> Redis() => _redis.GetLocalizedStringAsync(Culture, Key);

    [Benchmark(Description = "MemoryCache (FrozenDictionary)")]
    public ValueTask<string?> MemoryCache() => _memory.GetLocalizedStringAsync(Culture, Key);
    [Benchmark(Description = "Hybrid ( Level1 -> Level 2 -> Level 3, warm)")]
    public ValueTask<string?> Hybrid_Warm() => _hybrid.GetLocalizedStringAsync(Culture, Key);
}

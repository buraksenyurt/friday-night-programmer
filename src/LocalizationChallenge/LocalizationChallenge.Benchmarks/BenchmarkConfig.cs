using BenchmarkDotNet.Columns;
using BenchmarkDotNet.Configs;
using BenchmarkDotNet.Diagnosers;
using BenchmarkDotNet.Environments;
using BenchmarkDotNet.Jobs;
using BenchmarkDotNet.Exporters;
using BenchmarkDotNet.Loggers;
using BenchmarkDotNet.Order;

namespace LocalizationChallenge.Benchmarks;

// BenchmarkDotNet konfigürasyonu için özel bir sınıf oluşturuyoruz. 
// Bu sınıf, benchmark'larımızın nasıl çalışacağını ve hangi ölçümleri yapacağını belirlemekte
public class BenchmarkConfig
    : ManualConfig
{
    public BenchmarkConfig()
    {
        AddJob(Job.Default
            .WithRuntime(CoreRuntime.Core10_0) // Testlerimiz .NET 10 çalışma zamanında koşulacak
            .WithWarmupCount(3) // Her benchmark için 3 kez ısınma turu yapılacak, 
            // böylece JIT derlemesi ve diğer başlangıç maliyetleri hesaplamalara katılmaz. 
            .WithIterationCount(10) // Isınma turları bittikten sonra her benchmark için 10 ölçüm turu yapılacağını belirtmiş oluruz
            .WithInvocationCount(112)); // Her bir iterasyonda (turda) ilgili metotlar arka arkaya 112 kez çağrılacak.(Sadece 6nın katı olması gerektiğini belirten bir hata mesajına istinaden böyle yaptım)

        AddLogger(ConsoleLogger.Default); // Terminal ekranına log basılmasını sağlar.
        AddExporter(MarkdownExporter.GitHub); // Test sonuçlarını GitHub Markdown formatında dışa aktarılmasını sağlar.
        AddDiagnoser(MemoryDiagnoser.Default); // Sadece çalışmas süresinin değil, ne kadar RAM tüketildiğinin ve Garbage Collector'un ne kadar meşgul edildiğinin bilgileri de toplanır.
        AddColumnProvider(DefaultColumnProviders.Instance); // Rapor çıktısına eklenecen kolon adlarını belirler. (Örneğin, ortalama süre, standart sapma, bellek kullanımı gibi)
        Orderer = new DefaultOrderer(SummaryOrderPolicy.FastestToSlowest); // Sonuç tablosu en hızlı metotdan en yavaşa doğru sıralanır.
        Options |= ConfigOptions.JoinSummary;
    }
}

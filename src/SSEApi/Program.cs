using Serilog;
using Serilog.Sinks.SystemConsole.Themes;
using SSEApi;

Log.Logger = new LoggerConfiguration()
    .MinimumLevel.Information()
    .WriteTo.Console(
        outputTemplate: "[{Timestamp:HH:mm:ss} {Level:u3}] {SourceContext}{NewLine}  {Message:lj}{NewLine}{Exception}",
        theme: AnsiConsoleTheme.Code)
    .CreateLogger();

var builder = WebApplication.CreateBuilder(args);
builder.Host.UseSerilog();
builder.Services.AddSingleton<GamerService>();
builder.Services.AddHostedService<GamerServiceWorker>(); 

var app = builder.Build();
app.UseDefaultFiles().UseStaticFiles();
 
// GamerService bileşeninin SubscribeAsync metodunu kullanarak 
// abone olan istemcilere güncellemeleri ileten bir server-sent events endpoint'i olarak tanımlanır.
app.MapGet("/gamers/top/stream", (GamerService gamerService, CancellationToken cancellationToken) =>
{
    // TypedResults.ServerSentEvents() metodu, 
    // SubscribeAsync() metodundan dönen IAsyncEnumerable<List<Gamer>> türündeki sonuçları 
    // SSE formatına dönüştürerek istemcilere iletir.
    return TypedResults.ServerSentEvents(
        gamerService.SubscribeAsync(cancellationToken),
        eventType: "top-gamers" // İsteğe bağlı olarak, her bir SSE mesajına "top-gamers" şeklinde bir flag eklenir.
    );
});

await app.RunAsync();

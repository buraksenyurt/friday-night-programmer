using NBomber.Contracts;
using NBomber.CSharp;
using NBomber.Http.CSharp;
using System.Text.Json;

namespace LocalizationChallenge.LoadTest;

class Program
{
    static void Main()
    {
        var postgresMetric = Metric.CreateGauge("postgres_read_us", unitOfMeasure: "us");
        var redisMetric = Metric.CreateGauge("redis_read_us", unitOfMeasure: "us");
        var memoryMetric = Metric.CreateGauge("memory_read_us", unitOfMeasure: "us");
        var hybridMetric = Metric.CreateGauge("hybrid_read_us", unitOfMeasure: "us");

        var cultures = new[] { "tr-TR", "en-US", "de-DE" };
        var keys = new[] { "welcome_message", "farewell_message", "button_save", "error_not_found" };

        var handler = new HttpClientHandler
        {
            ServerCertificateCustomValidationCallback = HttpClientHandler.DangerousAcceptAnyServerCertificateValidator
        };
        var httpClient = new HttpClient(handler);

        var scenario = Scenario.Create("ramp_up_scenario", async context =>
        {
            var culture = cultures[Random.Shared.Next(cultures.Length)];
            var key = keys[Random.Shared.Next(keys.Length)];

            var request = Http.CreateRequest("GET", $"https://localhost:7092/api/benchmark/{culture}/{key}");
            var response = await Http.Send(httpClient, request);

            if (response.StatusCode != "OK" || response.Payload.Value == null)
            {
                return Response.Fail(statusCode: response.StatusCode);
            }

            try
            {
                using var httpResponse = response.Payload.Value;
                await using var stream = await httpResponse.Content.ReadAsStreamAsync();
                using var jsonDoc = await JsonDocument.ParseAsync(stream);

                if (jsonDoc.RootElement.TryGetProperty("results", out var results))
                {
                    foreach (var result in results.EnumerateArray())
                    {
                        var provider = result.GetProperty("providerName").GetString();
                        var elapsed = result.GetProperty("elapsedMicroseconds").GetDouble();

                        switch (provider)
                        {
                            case "Postgres": postgresMetric.Set(elapsed); break;
                            case "Redis": redisMetric.Set(elapsed); break;
                            case "InMemoryCache": memoryMetric.Set(elapsed); break;
                            case "Hybrid": hybridMetric.Set(elapsed); break;
                        }
                    }
                }

                return Response.Ok(statusCode: response.StatusCode);
            }
            catch (Exception ex)
            {
                context.Logger.Error(ex, "JSON parse error has been occured.");
                return Response.Fail(statusCode: response.StatusCode, "Parse Error");
            }
        })
        .WithInit(ctx =>
        {
            ctx.RegisterMetric(postgresMetric);
            ctx.RegisterMetric(redisMetric);
            ctx.RegisterMetric(memoryMetric);
            ctx.RegisterMetric(hybridMetric);
            return Task.CompletedTask;
        })
        .WithWarmUpDuration(TimeSpan.FromSeconds(5))
        .WithLoadSimulations(
            Simulation.RampingConstant(copies: 100, during: TimeSpan.FromSeconds(30)),
            Simulation.KeepConstant(copies: 100, during: TimeSpan.FromSeconds(60)),
            Simulation.RampingConstant(copies: 0, during: TimeSpan.FromSeconds(10))
        );

        NBomberRunner
            .RegisterScenarios(scenario)
            .Run();
    }
}
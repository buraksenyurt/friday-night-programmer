using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;
using CallMeSdk.Providers.Factories;
using Microsoft.Extensions.Logging;

namespace CallMeSdk.Client.Terminal;

internal class Program
{
    static async Task Main()
    {
        using ILoggerFactory loggerFactory = LoggerFactory.Create(builder => builder.AddConsole());
        ILogger logger = loggerFactory.CreateLogger("Program");


        logger.LogInformation("Application started...");

        var apiConfig = new RestConfiguration
        {
            BaseUrl = new Uri("https://api.azonbank.com"),
            Endpoint = "/customers"
        };
        var factory = new RestDataProviderFactory(new HttpClient(), apiConfig, new AzonBankCustomerDataAdapter(), logger);
        var provider = factory.GetDataProvider();

        var customers = await provider.FetchAsync();
        logger.LogInformation("{Count} customer found", customers.Count());
        Console.WriteLine(customers.Count());
    }
}

public class AzonBankCustomerDataAdapter
    : ICustomerDataParser
{
    public IEnumerable<CustomerBase> ConvertFrom(string rawData)
    {
        return [];
    }
}

using CallMeSdk.Core.Models;
using CallMeSdk.Providers.Factories;

namespace CallMeSdk.Client.Terminal;

internal class Program
{
    static async Task Main()
    {
        var apiConfig = new RestConfiguration
        {
            BaseUrl = new Uri("https://api.azonbank.com"),
            Endpoint = "/customers"
        };

        var httpClient = new HttpClient();
        var factory = new RestDataProviderFactory(apiConfig, httpClient);
        var provider = factory.GetDataProvider();

        var customers = await provider.FetchAsync();
        Console.WriteLine(customers.Count());
    }
}

using CallMeSdk.Adapters;
using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;
using CallMeSdk.Providers.Providers;

namespace CallMeSdk.Providers.Factories;

public class RestDataProviderFactory(RestConfiguration apiConfiguration, HttpClient httpClient)
        : DataProviderFactory
{
    private readonly RestConfiguration _apiConfiguration = apiConfiguration;
    private readonly HttpClient _httpClient = httpClient;

    public override IDataProvider GetDataProvider()
    {
        return new RestDataProvider(new RestCustomerDataAdapter(), _httpClient, _apiConfiguration);
    }
}

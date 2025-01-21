using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;
using CallMeSdk.Providers.Providers;
using Microsoft.Extensions.Logging;

namespace CallMeSdk.Providers.Factories;

public class RestDataProviderFactory(HttpClient httpClient, RestConfiguration apiConfiguration, ICustomerDataParser dataParser, ILogger logger)
        : DataProviderFactory
{
    private readonly RestConfiguration _apiConfiguration = apiConfiguration;
    private readonly ICustomerDataParser _dataParser = dataParser;
    private readonly ILogger _logger = logger;
    private readonly HttpClient _httpClient = httpClient;

    public override IDataProvider GetDataProvider()
    {
        return new RestDataProvider(_dataParser, _httpClient, _apiConfiguration, _logger);
    }
}

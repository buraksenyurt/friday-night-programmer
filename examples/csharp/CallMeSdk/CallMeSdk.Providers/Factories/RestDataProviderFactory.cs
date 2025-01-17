using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;
using CallMeSdk.Providers.Providers;
using Microsoft.Extensions.Logging;

namespace CallMeSdk.Providers.Factories;

public class RestDataProviderFactory(RestConfiguration apiConfiguration, ICustomerDataParser dataParser, ILogger logger)
        : DataProviderFactory
{
    private readonly RestConfiguration _apiConfiguration = apiConfiguration;
    private readonly ICustomerDataParser _dataParser = dataParser;
    private readonly ILogger _logger = logger;

    public override IDataProvider GetDataProvider()
    {
        return new RestDataProvider(_dataParser, _apiConfiguration, _logger);
    }
}

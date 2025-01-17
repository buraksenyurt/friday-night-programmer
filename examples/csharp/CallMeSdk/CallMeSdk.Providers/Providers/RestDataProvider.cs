using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;
using Microsoft.Extensions.Logging;

namespace CallMeSdk.Providers.Providers;

public class RestDataProvider(ICustomerDataParser customerDataParser,HttpClient httpClient, RestConfiguration apiConfiguration, ILogger logger)
        : IDataProvider
{
    private readonly ICustomerDataParser _customerDataParser = customerDataParser;
    private readonly RestConfiguration _apiConfiguration = apiConfiguration;
    private readonly HttpClient _httpClient=httpClient;
    private readonly ILogger _logger = logger;

    public async Task<IEnumerable<CustomerBase>> FetchAsync()
    {
        var apiUrl = _apiConfiguration.BaseUrl + _apiConfiguration.Endpoint;
        _logger.LogInformation("fetching {}", apiUrl);
        try
        {
            var response = await _httpClient.GetAsync(apiUrl);
            response.EnsureSuccessStatusCode();

            var jsonResponse = await response.Content.ReadAsStringAsync();
            _logger.LogInformation("json parsing completed");
            return _customerDataParser.ConvertFrom(jsonResponse);
        }
        catch (HttpRequestException ex)
        {
            _logger.LogError("Error on data fetch.Exception message is : {}", ex.Message);
            return [];
        }
    }
}

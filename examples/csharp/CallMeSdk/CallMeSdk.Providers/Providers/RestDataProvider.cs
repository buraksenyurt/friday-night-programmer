using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;

namespace CallMeSdk.Providers.Providers;

public class RestDataProvider(ICustomerDataAdapter customerDataAdapter, HttpClient httpClient, RestConfiguration apiConfiguration)
        : IDataProvider
{
    private readonly ICustomerDataAdapter _customerDataAdapter = customerDataAdapter;
    private readonly HttpClient _httpClient = httpClient;
    private readonly RestConfiguration _apiConfiguration = apiConfiguration;

    public async Task<IEnumerable<CustomerBase>> FetchAsync()
    {
        var apiUrl = _apiConfiguration.BaseUrl + _apiConfiguration.Endpoint;
        try
        {
            var response = await _httpClient.GetAsync(apiUrl);
            response.EnsureSuccessStatusCode();

            var jsonResponse = await response.Content.ReadAsStringAsync();
            return _customerDataAdapter.ConvertFrom(jsonResponse);
        }
        catch (HttpRequestException ex)
        {
            //TODO@buraksenyurt Logger üstünden hata basılmalı
            return [];
        }
    }
}

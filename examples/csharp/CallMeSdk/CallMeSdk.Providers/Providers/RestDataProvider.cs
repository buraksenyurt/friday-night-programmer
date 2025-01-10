using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;

namespace CallMeSdk.Providers.Providers;

public class RestDataProvider 
    : IDataProvider
{
    private readonly ICustomerDataAdapter _customerDataAdapter;
    public RestDataProvider(ICustomerDataAdapter customerDataAdapter)
    {
        _customerDataAdapter = customerDataAdapter;
    }
    public IEnumerable<CustomerBase> Fetch()
    {
        //TODO@buraksenyurt sampleData servis'ten geliyor olmalı
        var sampleData = "{ \"customers\": [{ \"Id\": \"112345\", \"Fullname\": \"John Doe\", \"Email\": \"john.doe@azon.com\" }] }";
        return _customerDataAdapter.ConvertFrom(sampleData);
    }
}

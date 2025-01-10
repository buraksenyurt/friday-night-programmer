using CallMeSdk.Adapters;
using CallMeSdk.Core.Interfaces;
using CallMeSdk.Providers.Providers;

namespace CallMeSdk.Providers.Factories;

public class RestDataProviderFactory
    : DataProviderFactory
{
    public override IDataProvider GetDataProvider()
    {
        return new RestDataProvider(new RestCustomerDataAdapter());
    }
}

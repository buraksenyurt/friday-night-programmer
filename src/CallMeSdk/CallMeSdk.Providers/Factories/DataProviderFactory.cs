using CallMeSdk.Core.Interfaces;

namespace CallMeSdk.Providers.Factories;

public abstract class DataProviderFactory
{
    public abstract IDataProvider GetDataProvider();
}

using CallMeSdk.Core.Interfaces;

namespace CallMeSdk.Providers.Factories;

public abstract class DataProviderFactory
{
    protected abstract IDataProvider GetDataProvider();
}

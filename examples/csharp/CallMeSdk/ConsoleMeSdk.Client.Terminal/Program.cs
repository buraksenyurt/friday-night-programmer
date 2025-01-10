using CallMeSdk.Providers.Factories;

namespace CallMeSdk.Client.Terminal;

internal class Program
{
    static void Main()
    {
        var restFactory = new RestDataProviderFactory();
        var restProvider = restFactory.GetDataProvider();

        var azonBankCustomers = restProvider.Fetch();
    }
}

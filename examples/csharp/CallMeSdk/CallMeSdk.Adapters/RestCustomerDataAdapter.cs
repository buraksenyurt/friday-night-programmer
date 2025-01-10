using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;

namespace CallMeSdk.Adapters;

public class RestCustomerDataAdapter
    : ICustomerDataAdapter
{
    public IEnumerable<CustomerBase> ConvertFrom(string rawData)
    {
        throw new NotImplementedException();
    }
}

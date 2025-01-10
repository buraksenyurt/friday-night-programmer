using CallMeSdk.Core.Models;

namespace CallMeSdk.Core.Interfaces;

public interface ICustomerDataAdapter
{
    IEnumerable<CustomerBase> ConvertFrom(string rawData);
}

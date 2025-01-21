using CallMeSdk.Core.Models;

namespace CallMeSdk.Core.Interfaces;

public interface ICustomerDataParser
{
    IEnumerable<CustomerBase> ConvertFrom(string rawData);
}

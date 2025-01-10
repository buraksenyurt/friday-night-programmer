using CallMeSdk.Core.Models;

namespace CallMeSdk.Core.Interfaces;

public interface IDataProvider
{
    IEnumerable<CustomerBase> Fetch();
}

using CallMeSdk.Core.Models;

namespace CallMeSdk.Core.Interfaces;

public interface IDataProvider
{
    Task<IEnumerable<CustomerBase>> FetchAsync();
}

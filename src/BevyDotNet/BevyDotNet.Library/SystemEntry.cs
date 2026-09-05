namespace BevyDotNet.Library;

internal class SystemEntry(object system)
{
    public object System { get; set; } = system;
    public HashSet<Type> RunBefore { get; } = [];
    public HashSet<Type> RunAfter { get; } = [];
}

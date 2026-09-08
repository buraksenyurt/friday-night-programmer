namespace BevyDotNet.Library;

public partial class Scheduler
{
    internal readonly Logger Logger = new();
    public void SetLogLevel(LogLevel level) => Logger.DefaultLevel = level;
}

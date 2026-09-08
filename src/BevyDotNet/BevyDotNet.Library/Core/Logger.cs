namespace BevyDotNet.Library;

public class Logger
{
    public LogLevel DefaultLevel { get; set; } = LogLevel.Info;

    private void Write(LogLevel level, string message)
    {
        if (level < DefaultLevel) return;

        var color = level switch
        {
            LogLevel.Trace => ConsoleColor.Gray,
            LogLevel.Debug => ConsoleColor.Cyan,
            LogLevel.Info => ConsoleColor.Green,
            LogLevel.Warn => ConsoleColor.Yellow,
            LogLevel.Error => ConsoleColor.Red,
            _ => ConsoleColor.White
        };

        var previousColor = Console.ForegroundColor;
        Console.ForegroundColor = color;
        Console.WriteLine($"[{level,-5}] {message}");
        Console.ForegroundColor = previousColor;
    }

    public void Trace(string message) => Write(LogLevel.Trace, message);
    public void Debug(string message) => Write(LogLevel.Debug, message);
    public void Info(string message) => Write(LogLevel.Info, message);
    public void Warn(string message) => Write(LogLevel.Warn, message);
    public void Error(string message) => Write(LogLevel.Error, message);
}
